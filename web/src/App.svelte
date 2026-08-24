<script lang="ts">
  import { onMount } from 'svelte';
  import {
    WebCramSession,
    loadCatalog,
    renderAnswer,
    searchCatalog,
    summarizeProgress
  } from './lib/core';
  import {
    downloadProgress,
    emptyProgress,
    loadProgress,
    readProgressFile,
    saveProgress
  } from './lib/progress';
  import type {
    Card,
    CardSet,
    Catalog,
    ProgressFile,
    ProgressSummary,
    ReviewRating,
    SearchHit
  } from './lib/types';

  type View = 'library' | 'set' | 'card' | 'cram' | 'progress';

  const repositoryUrl = 'https://github.com/julian-corbet/nixcards-corbet-ch';

  let catalog = $state<Catalog>({ sets: [] });
  let progress = $state<ProgressFile>(emptyProgress());
  let summary = $state<ProgressSummary>({
    reviewed_cards: 0,
    known_cards: 0,
    review_events: 0,
    latest: {}
  });
  let view = $state<View>('library');
  let selectedSetId = $state('');
  let selectedCardId = $state('');
  let searchQuery = $state('');
  let searchHits = $state<SearchHit[]>([]);
  let revealed = $state(false);
  let answerHtml = $state('');
  let loading = $state(true);
  let error = $state('');
  let notice = $state('');
  let cramRemaining = $state(0);
  let cramInitial = $state(0);
  let cramReviews = $state(0);
  let cramComplete = $state(false);
  let importInput: HTMLInputElement;
  let cramSession: WebCramSession | null = null;

  let selectedSet = $derived(catalog.sets.find((set) => set.id === selectedSetId));
  let selectedCard = $derived(selectedSet?.cards.find((card) => card.id === selectedCardId));

  onMount(async () => {
    try {
      catalog = await loadCatalog();
      progress = await loadProgress();
      refreshSummary();
    } catch (caught) {
      error = caught instanceof Error ? caught.message : String(caught);
    } finally {
      loading = false;
    }
  });

  function refreshSummary(): void {
    summary = summarizeProgress(progress);
  }

  function showLibrary(): void {
    view = 'library';
    revealed = false;
    notice = '';
  }

  function openSet(setId: string): void {
    selectedSetId = setId;
    selectedCardId = '';
    view = 'set';
    revealed = false;
    notice = '';
  }

  function openCard(setId: string, cardId: string): void {
    selectedSetId = setId;
    selectedCardId = cardId;
    view = 'card';
    revealed = false;
    answerHtml = '';
  }

  function revealCard(): void {
    if (!selectedCard) return;
    answerHtml = renderAnswer(selectedSetId, selectedCard.id);
    revealed = true;
  }

  function moveCard(delta: number): void {
    if (!selectedSet || !selectedCard) return;
    const index = selectedSet.cards.findIndex((card) => card.id === selectedCard.id);
    const next = (index + delta + selectedSet.cards.length) % selectedSet.cards.length;
    openCard(selectedSet.id, selectedSet.cards[next].id);
  }

  function editCardUrl(card: Card): string {
    return `${repositoryUrl}/edit/cards/${card.source_path.replace(/^cards\//, '')}`;
  }

  function updateSearch(event: Event): void {
    searchQuery = (event.currentTarget as HTMLInputElement).value;
    searchHits = searchQuery.trim() ? searchCatalog(searchQuery) : [];
  }

  function startCram(set: CardSet): void {
    cramSession?.free();
    cramSession = new WebCramSession(set.id, Date.now() >>> 0);
    selectedSetId = set.id;
    cramInitial = cramSession.initial_count();
    cramComplete = false;
    view = 'cram';
    syncCramCard();
  }

  function syncCramCard(): void {
    if (!cramSession || cramSession.is_complete()) {
      cramComplete = true;
      cramRemaining = 0;
      revealed = false;
      selectedCardId = '';
      return;
    }
    const canonical = cramSession.current();
    selectedCardId = canonical?.split('#', 2)[1] ?? '';
    cramRemaining = cramSession.remaining();
    cramReviews = cramSession.reviews();
    revealed = false;
    answerHtml = '';
  }

  async function rateCard(rating: ReviewRating): Promise<void> {
    if (!cramSession || !revealed) return;
    const reviewedCard = cramSession.answer(rating);
    progress.events.push({
      card_id: reviewedCard,
      rating,
      reviewed_at: Math.floor(Date.now() / 1000)
    });
    progress = { ...progress, events: [...progress.events] };
    await saveProgress(progress);
    if ('storage' in navigator && 'persist' in navigator.storage) {
      void navigator.storage.persist();
    }
    refreshSummary();
    syncCramCard();
  }

  async function importProgress(event: Event): Promise<void> {
    const input = event.currentTarget as HTMLInputElement;
    const file = input.files?.[0];
    if (!file) return;
    try {
      const imported = await readProgressFile(file);
      summarizeProgress(imported);
      await saveProgress(imported);
      progress = imported;
      refreshSummary();
      notice = `Imported ${imported.events.length} review events.`;
    } catch (caught) {
      notice = caught instanceof Error ? caught.message : String(caught);
    } finally {
      input.value = '';
    }
  }

  function known(card: Card): boolean {
    return summary.latest[card.canonical_id] === 'known';
  }
</script>

<svelte:head>
  <meta property="og:title" content="nixcards — local-first flashcards" />
  <meta
    property="og:description"
    content="Browse curated Markdown flashcards and cram locally, without an account."
  />
  <meta property="og:type" content="website" />
  <meta property="og:url" content="https://nixcards.corbet.ch/" />
</svelte:head>

<div class="app-shell">
  <header class="topbar">
    <button class="wordmark" onclick={showLibrary} aria-label="Open library">nixcards</button>
    <div class="progress-chip" aria-label={`${summary.known_cards} known cards`}>
      <span>{summary.known_cards}</span> known
    </div>
  </header>

  <main>
    {#if loading}
      <section class="center-state" aria-live="polite">
        <div class="loader"></div>
        <p>Loading the catalogue…</p>
      </section>
    {:else if error}
      <section class="center-state error-state" role="alert">
        <h1>Could not load nixcards</h1>
        <p>{error}</p>
      </section>
    {:else if view === 'library'}
      <section class="screen library-screen">
        <div class="intro">
          <p class="eyebrow">Local-first · no account</p>
          <h1>What do you want to remember?</h1>
          <p>Browse every card freely or start a focused cram round.</p>
        </div>

        <label class="search-box">
          <span class="sr-only">Search all cards</span>
          <span aria-hidden="true">⌕</span>
          <input
            type="search"
            placeholder="Search questions and answers"
            value={searchQuery}
            oninput={updateSearch}
          />
        </label>

        {#if searchQuery.trim()}
          <div class="section-heading">
            <h2>Results</h2>
            <span>{searchHits.length}</span>
          </div>
          <div class="card-list">
            {#each searchHits as hit (hit.canonical_id)}
              <button class="card-row" onclick={() => openCard(hit.set_id, hit.card_id)}>
                <span class="card-row-copy">
                  <small>{hit.set_title}</small>
                  <strong>{hit.question}</strong>
                </span>
                <span aria-hidden="true">›</span>
              </button>
            {:else}
              <p class="empty-state">No cards match that search.</p>
            {/each}
          </div>
        {:else}
          <div class="section-heading">
            <h2>Card sets</h2>
            <span>{catalog.sets.length}</span>
          </div>
          <div class="set-grid">
            {#each catalog.sets as set (set.id)}
              <article class="set-card">
                <button class="set-open" onclick={() => openSet(set.id)}>
                  <div class="set-symbol" aria-hidden="true">{set.title.slice(0, 1)}</div>
                  <div>
                    <small>{set.id}</small>
                    <h3>{set.title}</h3>
                    <p>{set.cards.length} cards · {set.language.toUpperCase()}</p>
                  </div>
                  <span aria-hidden="true">›</span>
                </button>
                <button class="cram-shortcut" onclick={() => startCram(set)}>Start cram</button>
              </article>
            {/each}
          </div>
        {/if}
      </section>
    {:else if view === 'set' && selectedSet}
      <section class="screen">
        <button class="back-button" onclick={showLibrary}>← Library</button>
        <div class="set-hero">
          <p class="eyebrow">{selectedSet.id}</p>
          <h1>{selectedSet.title}</h1>
          <div class="tag-row">
            {#each selectedSet.tags as tag}
              <span>{tag}</span>
            {/each}
          </div>
          <button class="primary-action" onclick={() => startCram(selectedSet)}>
            Start {selectedSet.cards.length}-card cram
          </button>
        </div>

        <div class="section-heading">
          <h2>Browse freely</h2>
          <span>Does not affect progress</span>
        </div>
        <div class="card-list numbered">
          {#each selectedSet.cards as card, index (card.id)}
            <button class="card-row" onclick={() => openCard(selectedSet.id, card.id)}>
              <span class:known={known(card)} class="card-number">
                {known(card) ? '✓' : index + 1}
              </span>
              <span class="card-row-copy"><strong>{card.question}</strong></span>
              <span aria-hidden="true">›</span>
            </button>
          {/each}
        </div>
      </section>
    {:else if view === 'card' && selectedSet && selectedCard}
      <section class="screen card-screen">
        <button class="back-button" onclick={() => openSet(selectedSet.id)}>← {selectedSet.title}</button>
        <div class="card-position">
          {selectedSet.cards.findIndex((card) => card.id === selectedCard.id) + 1}
          / {selectedSet.cards.length}
        </div>
        <article class="study-card" class:revealed>
          <p class="eyebrow">Question</p>
          <h1>{selectedCard.question}</h1>
          {#if revealed}
            <div class="answer" aria-live="polite">{@html answerHtml}</div>
          {:else}
            <button class="reveal-action" onclick={revealCard}>Reveal answer</button>
          {/if}
        </article>
        <div class="browse-controls">
          <button onclick={() => moveCard(-1)}>← Previous</button>
          <button onclick={() => moveCard(1)}>Next →</button>
        </div>
        <p class="browse-note">Browsing never changes your learning progress.</p>
        <a class="edit-card-link" href={editCardUrl(selectedCard)} target="_blank" rel="noreferrer">
          Edit this card on GitHub ↗
        </a>
      </section>
    {:else if view === 'cram' && selectedSet}
      <section class="screen card-screen cram-screen">
        <button class="back-button" onclick={() => openSet(selectedSet.id)}>× End cram</button>
        <div class="cram-meter" aria-label={`${cramRemaining} of ${cramInitial} cards remaining`}>
          <span style={`width: ${cramInitial ? ((cramInitial - cramRemaining) / cramInitial) * 100 : 100}%`}></span>
        </div>
        {#if cramComplete}
          <article class="completion-card">
            <div class="completion-mark">✓</div>
            <p class="eyebrow">Round complete</p>
            <h1>You cleared the set.</h1>
            <p>{cramReviews} reviews for {cramInitial} cards.</p>
            <button class="primary-action" onclick={() => startCram(selectedSet)}>Run it again</button>
            <button class="secondary-action" onclick={() => openSet(selectedSet.id)}>Browse set</button>
          </article>
        {:else if selectedCard}
          <div class="cram-meta">
            <span>{cramRemaining} remaining</span>
            <span>{cramReviews} reviewed</span>
          </div>
          <article class="study-card" class:revealed>
            <p class="eyebrow">Question</p>
            <h1>{selectedCard.question}</h1>
            {#if revealed}
              <div class="answer" aria-live="polite">{@html answerHtml}</div>
            {:else}
              <button class="reveal-action" onclick={revealCard}>Reveal answer</button>
            {/if}
          </article>
          {#if revealed}
            <div class="rating-controls">
              <button class="again" onclick={() => rateCard('again')}>
                <small>Missed it</small>
                Again
              </button>
              <button class="known-button" onclick={() => rateCard('known')}>
                <small>Got it</small>
                Known
              </button>
            </div>
          {/if}
        {/if}
      </section>
    {:else if view === 'progress'}
      <section class="screen progress-screen">
        <div class="intro compact">
          <p class="eyebrow">Stored on this device</p>
          <h1>Your progress</h1>
          <p>Export a portable backup whenever you want.</p>
        </div>
        <div class="stats-grid">
          <article><strong>{summary.known_cards}</strong><span>known cards</span></article>
          <article><strong>{summary.reviewed_cards}</strong><span>reviewed cards</span></article>
          <article><strong>{summary.review_events}</strong><span>total reviews</span></article>
        </div>
        <div class="data-actions">
          <button class="primary-action" onclick={() => downloadProgress(progress)}>Export progress</button>
          <button class="secondary-action" onclick={() => importInput.click()}>Import progress</button>
          <input
            class="sr-only"
            bind:this={importInput}
            type="file"
            accept="application/json,.json"
            onchange={importProgress}
          />
        </div>
        {#if notice}<p class="notice" aria-live="polite">{notice}</p>{/if}
        <div class="privacy-note">
          <h2>No account. No tracking.</h2>
          <p>
            Cards ship with the app. Reviews stay in this browser unless you export them. Clearing
            site data also clears local progress.
          </p>
        </div>
        <footer>
          <a href={repositoryUrl}>Source</a>
          <span>FSL code · CC-BY-NC-SA cards</span>
        </footer>
      </section>
    {/if}
  </main>

  {#if !loading && !error && view !== 'cram'}
    <nav class="bottom-nav" aria-label="Primary">
      <button class:active={view !== 'progress'} onclick={showLibrary}>
        <span aria-hidden="true">▦</span>
        Library
      </button>
      <button class:active={view === 'progress'} onclick={() => (view = 'progress')}>
        <span aria-hidden="true">◔</span>
        Progress
      </button>
    </nav>
  {/if}
</div>
