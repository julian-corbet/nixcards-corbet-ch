---
id: cloud.bearingpoint.interview.senior-consultant
title: BearingPoint Cloud & GenAI Senior Consultant Interview
language: de
license: CC-BY-NC-SA-4.0
attribution: nixcards contributors
tags: [cloud, genai, data, consulting, interview, bearingpoint]
sources: [https://www.bearingpoint.com/en-ch/careers/open-roles/agebote/?country=CH&id=T4992684, https://www.bearingpoint.com/de-ch/dienstleistungen/technology/cloud/, https://learn.microsoft.com/azure/well-architected/, https://docs.aws.amazon.com/wellarchitected/latest/framework/welcome.html, https://kubernetes.io/docs/concepts/, https://genai.owasp.org/llm-top-10/]
---

## Was sucht BearingPoint laut der konkreten Ausschreibung? {#role-expectation}

Eine Person mit mehr als zwei Jahren Erfahrung in Data Analytics und AI, die kundenspezifische
GenAI-Lösungen entwirft und umsetzt, Use Cases in technische Anforderungen übersetzt, Junioren
anleitet und das Serviceportfolio weiterentwickelt. Dazu kommen Kundenorientierung,
Eigenverantwortung, agile Zusammenarbeit sowie verhandlungssicheres Deutsch und Englisch.

## Welche Projekterfahrung sollte ich zuerst erzählen? {#project-selection}

Wähle ein Projekt, in dem du nicht nur Technologie eingesetzt, sondern ein **unklares
Geschäftsproblem in eine produktionsfähige Lösung übersetzt** hast. Zeige dabei:

- Ausgangslage und messbares Ziel
- deine persönliche Verantwortung
- Architektur- und Trade-off-Entscheidungen
- Security, Governance und Betrieb
- Ergebnis und konkrete Wirkung

## Wie strukturiere ich eine Projekterfahrung im Interview? {#project-story-structure}

Nutze eine knappe Folge: **Kontext → Problem → Auftrag → Entscheidung → Umsetzung → Ergebnis →
Lernpunkt**. Sprich in der Ich-Form über deinen Beitrag und quantifiziere Wirkung, Zeit, Qualität
oder Risiko, wo du belastbare Zahlen hast.

## Was unterscheidet einen Senior Consultant von einem reinen Implementierer? {#senior-consultant}

Ein Senior Consultant verbindet Business-Ziel, Stakeholder, Architektur und Delivery. Er macht
Annahmen sichtbar, strukturiert Unsicherheit, begründet Entscheidungen, befähigt andere und sorgt
dafür, dass die Lösung nach dem Projekt sicher betrieben und weiterentwickelt werden kann.

## Wie übersetzt man einen GenAI-Use-Case in technische Anforderungen? {#use-case-to-requirements}

Beginne mit Nutzer, Entscheidung und gewünschtem Ergebnis. Definiere dann Datenquellen,
Qualitätsmaß, Latenz, Volumen, Berechtigungen, Datenschutz, Integrationen, Betriebsmodell und
Fehlergrenzen. Erst danach wählst du Modell, Retrieval, Hosting und konkrete Cloud-Dienste.

## Welche Frage muss vor jeder GenAI-Architektur beantwortet werden? {#genai-first-question}

**Welches überprüfbare Problem lösen wir besser als mit einer einfacheren Lösung?** Ohne klare
Baseline und Erfolgskriterium kann ein überzeugender Prototyp trotzdem ein schlechtes Produkt
sein.

## Wann ist RAG sinnvoll? {#rag-when}

Wenn Antworten aktuelles oder organisationsspezifisches Wissen benötigen, Quellen sichtbar sein
sollen und das Modell nicht neu trainiert werden muss. RAG ersetzt aber weder Berechtigungen noch
Qualitätssicherung und ist unnötig, wenn eine deterministische Suche oder Regel genügt.

## Wie sieht eine belastbare RAG-Architektur aus? {#rag-architecture}

Quellen → Parsing/OCR → Normalisierung → Chunking → Embeddings und Suchindex → Retrieval →
optionales Reranking → Prompt mit Kontext → Modell → Antwort mit Quellen. Quer dazu liegen
Identität, Berechtigungen, Evaluation, Observability und Schutz vor Prompt Injection.

## Wie bestimme ich Chunk-Größe und Überlappung? {#chunking}

Nach Dokumentstruktur und Fragetyp, nicht nach einer universellen Zahl. Ein Chunk muss genug
Kontext für eine Antwort enthalten, aber klein genug für präzises Retrieval bleiben. Überlappung
hilft an Grenzen, erhöht jedoch Indexgröße und Duplikate. Entscheidend ist ein Testset realer
Fragen.

## Vector Search, Keyword Search oder Hybrid Search? {#retrieval-choice}

- **Keyword:** stark bei exakten Begriffen, IDs und seltenen Namen
- **Vector:** stark bei semantisch ähnlichen Formulierungen
- **Hybrid:** kombiniert beide Signale und ist für heterogene Unternehmensdokumente oft robuster

Die Wahl wird mit Retrieval-Metriken am eigenen Korpus geprüft.

## Was ist Reranking? {#reranking}

Ein zweites Modell oder Verfahren bewertet eine kleine Menge gefundener Dokumente genauer und
ordnet sie neu. Retrieval maximiert zunächst Recall; Reranking verbessert anschließend die
Präzision der wenigen Chunks, die wirklich in den Prompt gelangen.

## Wie reduziert man Halluzinationen in RAG? {#hallucinations}

Gute Quellen und Retrieval-Qualität, klare Instruktionen, Antwort nur aus belegtem Kontext,
Quellenanzeige, ein sinnvoller Mindestscore, ehrliches Ablehnen bei fehlender Evidenz und
regelmäßige Evaluation. Ein größeres Modell allein löst schlechte Evidenz nicht.

## Wie misst man Retrieval- und Antwortqualität? {#rag-evaluation}

Mit einem versionierten Golden Test Set. Retrieval wird beispielsweise über Recall@k und
Precision@k gemessen. Antworten werden auf Faktentreue, Vollständigkeit, Quellenbezug,
Ablehnungsverhalten, Latenz und Kosten geprüft. Produktionsfeedback ergänzt, ersetzt aber nicht die
reproduzierbare Offline-Evaluation.

## Wie setzt man Dokumentberechtigungen in RAG durch? {#rag-permissions}

Die Nutzeridentität und ihre Zugriffsrechte müssen bereits das Retrieval filtern. Erst erlaubte
Chunks dürfen den Prompt erreichen. Ein Filter erst nach der Modellantwort ist zu spät, weil das
Modell den verbotenen Inhalt bereits verarbeitet hat.

## Was ist Prompt Injection im RAG-Kontext? {#prompt-injection}

Ein Dokument enthält Anweisungen, die das Modell statt der eigentlichen Systemregeln befolgen soll.
Dokumente werden daher als **nicht vertrauenswürdige Daten** behandelt: Instruktionen und Inhalt
trennen, Tools minimal berechtigen, Ausgaben validieren und sensible Aktionen außerhalb des
Modells autorisieren.

## Wie behandelt man personenbezogene oder vertrauliche Daten in GenAI? {#genai-sensitive-data}

Daten klassifizieren und minimieren, Rechtsgrundlage und Zweck klären, Aufbewahrung begrenzen,
Verschlüsselung und Zugriffskontrollen durchsetzen, Modell- und Providerverträge prüfen und
Auditierbarkeit schaffen. Besonders sensible Daten dürfen nicht stillschweigend in Prompts oder
Trainingspfade gelangen.

## Was ist eine Cloud Landing Zone? {#landing-zone}

Eine standardisierte, governte Ausgangsumgebung für Cloud-Workloads. Sie legt Identität,
Kontenstruktur, Netzwerk, Logging, Security, Policies, Kostenstellen und Automatisierung fest,
bevor Teams produktive Anwendungen darauf bauen.

## Warum beginnt eine Cloud-Transformation nicht mit der Migration einzelner Server? {#strategy-before-migration}

Ohne Zielbild, Governance und Betriebsmodell werden bestehende Probleme nur in eine andere
Umgebung verschoben. Zuerst werden Geschäftsnutzen, Risikohaltung, Verantwortlichkeiten,
Zielarchitektur und Migrationsroadmap geklärt.

## Wie strukturiert BearingPoint eine Cloud-Strategie? {#bearingpoint-cloud-approach}

Die veröffentlichte Vorgehensweise umfasst vier Schritte: **Scoping**, **Governance-Modell**,
**Cloud Opportunity Assessment** und eine **Executive Summary** für die Unternehmensleitung.
Danach folgen je nach Bedarf Providerauswahl, Eignungsprüfung, Migration und Transformation des
Betriebsmodells.

## Was ist der Unterschied zwischen Skalierbarkeit und Elastizität? {#scalability-elasticity}

Skalierbarkeit ist die Fähigkeit, mehr Last durch zusätzliche oder stärkere Ressourcen zu tragen.
Elastizität bedeutet, diese Ressourcen passend zur aktuellen Last automatisch hinzuzufügen und
wieder zu entfernen.

## IaaS, PaaS oder SaaS: Wie entscheide ich? {#service-models}

Nach benötigter Kontrolle und gewünschter Betriebsentlastung. IaaS gibt viel Kontrolle, verlangt
aber mehr Betrieb. PaaS reduziert Plattformarbeit, bringt dafür stärkere Provider-Abhängigkeiten.
SaaS eignet sich für standardisierbare Geschäftsfunktionen. Die richtige Ebene ist die höchste,
die Anforderungen und Exit-Risiko noch sinnvoll erfüllt.

## Public, Private, Hybrid oder Multi-Cloud? {#deployment-models}

Die Entscheidung folgt Anforderungen an Regulierung, Daten, Latenz, bestehende Systeme,
Fähigkeiten und Resilienz. Multi-Cloud ist kein Selbstzweck: Sie erhöht Portabilitätsoptionen, aber
auch Betriebs-, Security- und Kompetenzaufwand.

## Was bedeutet Shared Responsibility? {#shared-responsibility}

Provider und Kunde teilen Sicherheits- und Betriebsaufgaben. Der Provider schützt je nach
Servicemodell mehr der zugrunde liegenden Plattform; der Kunde bleibt unter anderem für Daten,
Identitäten, Konfiguration, Berechtigungen und die sichere Nutzung verantwortlich.

## Wie entwirft man hochverfügbare Cloud-Systeme? {#high-availability}

Fehlerdomänen bestimmen, kritische Komponenten redundant über Zonen verteilen, Zustände bewusst
behandeln, Timeouts und Retries begrenzen, Abhängigkeiten entkoppeln und Degradationspfade
vorsehen. Verfügbarkeit muss durch Tests und Betriebsdaten bewiesen werden.

## Was ist der Unterschied zwischen Backup und Disaster Recovery? {#backup-vs-dr}

Ein Backup stellt Daten wieder her. Disaster Recovery stellt einen definierten Geschäftsbetrieb
nach einem größeren Ausfall wieder her und umfasst Systeme, Abhängigkeiten, Reihenfolge,
Verantwortung und regelmäßig getestete Wiederanlaufverfahren.

## Was bedeuten RTO und RPO? {#rto-rpo}

- **RTO:** maximal akzeptierte Zeit bis zur Wiederherstellung des Betriebs
- **RPO:** maximal akzeptierter Datenverlust, als Zeitspanne gemessen

Beide sind Geschäftsanforderungen und bestimmen Architektur sowie Kosten.

## Wie sieht Least Privilege praktisch aus? {#least-privilege}

Menschen und Workloads erhalten nur die für eine konkrete Aufgabe nötigen Rechte, möglichst kurz
und kontextgebunden. Rollen statt Einzelberechtigungen, getrennte Admin-Pfade, regelmäßige Reviews
und Audit Logs machen das Prinzip betreibbar.

## Warum sind Workload-Identitäten besser als langlebige Zugangsschlüssel? {#workload-identity}

Sie ermöglichen kurzlebige, automatisch ausgestellte Berechtigungen für einen konkreten Workload.
Dadurch entfallen verteilte statische Secrets, Rotation wird einfacher und Zugriffe lassen sich
besser einem Laufzeitkontext zuordnen.

## Wie schützt man Secrets in der Cloud? {#secret-management}

In einem dedizierten Secret Manager speichern, verschlüsseln, Zugriff minimal halten, Rotation und
Auditierung automatisieren und Secrets niemals in Quellcode, Images, Logs oder allgemeine
Konfiguration schreiben.

## Welche Logging-Ebenen braucht eine regulierte Cloud-Plattform? {#audit-logging}

Unveränderliche Audit Logs der Control Plane, Identitäts- und Zugriffsereignisse, Netzwerk- und
Security-Signale sowie Anwendungs- und Datenzugriffslogs. Zentrale Sammlung, definierte
Aufbewahrung, Zeit-Synchronisation und Alarmierung sind Teil der Architektur.

## Was ist Zero Trust in einem Satz? {#zero-trust}

Kein Zugriff wird allein wegen Netzwerkstandort oder früherem Vertrauen akzeptiert; Identität,
Gerätezustand, Kontext und Berechtigung werden für jede relevante Aktion geprüft und kontinuierlich
neu bewertet.

## Wie segmentiert man ein Cloud-Netzwerk sinnvoll? {#network-segmentation}

Nach Vertrauensgrenzen und Kommunikationsbedarf, nicht nach einem einzigen großen internen Netz.
Workloads erhalten explizite erlaubte Pfade, sensible Dienste private Endpunkte und Internetzugriff
wird kontrolliert, beobachtet und auf das Notwendige beschränkt.

## Was ist Infrastructure as Code? {#infrastructure-as-code}

Infrastruktur wird deklarativ beschrieben, versioniert, geprüft und reproduzierbar ausgerollt.
Dadurch werden Änderungen reviewbar, Umgebungen konsistenter und Wiederaufbau sowie Auditierung
wesentlich zuverlässiger als bei manuellen Klickpfaden.

## Wie verhindert man Konfigurationsdrift? {#configuration-drift}

Eine deklarative Quelle wird zur Autorität, Änderungen laufen nur über Review und automatisierte
Pipelines, Policies prüfen Abweichungen und Drift wird erkannt oder zurückgesetzt. Manuelle
Notfalländerungen müssen anschließend bewusst in die Quelle zurückgeführt werden.

## Was gehört in eine sichere Cloud-CI/CD-Pipeline? {#secure-cicd}

Reproduzierbare Builds, minimale kurzlebige Berechtigungen, Secret-Schutz, Dependency- und
Artefaktprüfung, signierte oder nachvollziehbare Provenienz, Policy Gates, getrennte Umgebungen,
kontrollierte Promotion und ein getesteter Rollback- oder Roll-forward-Pfad.

## Was ist Observability im Unterschied zu Monitoring? {#observability}

Monitoring prüft bekannte Zustände und Schwellen. Observability liefert Logs, Metriken und Traces
so, dass auch unbekannte Fehlerzustände aus dem Verhalten des Systems untersucht werden können.
Beides benötigt klare Serviceziele und verantwortliche Reaktionen.

## Was sind SLI, SLO und SLA? {#sli-slo-sla}

- **SLI:** gemessener Indikator wie Verfügbarkeit oder Latenz
- **SLO:** internes Ziel für diesen Indikator
- **SLA:** vertragliche Zusage mit möglichen Konsequenzen

Ein Error Budget verbindet Zuverlässigkeit mit Änderungsgeschwindigkeit.

## Was ist FinOps? {#finops}

Eine gemeinsame Arbeitsweise von Engineering, Finance und Business, um Cloud-Nutzung transparent
zu machen und laufend nach Geschäftswert zu optimieren. Tags, Budgets und Rightsizing helfen, aber
Ownership und Entscheidungen sind wichtiger als ein einmaliger Kostenreport.

## Welche Migrationsstrategien sollte man kennen? {#migration-strategies}

Typische Optionen sind Rehost, Replatform, Refactor, Repurchase, Retire und Retain. Die Auswahl
erfolgt pro Anwendung anhand von Wert, Risiko, Abhängigkeiten, Zeit und Zielarchitektur statt durch
eine pauschale Cloud-first-Regel.

## Wann sind Container sinnvoll? {#containers}

Wenn Anwendungen reproduzierbar verpackt, isoliert und über Umgebungen konsistent betrieben werden
sollen. Container lösen aber weder Zustandsmanagement noch Security oder Orchestrierung
automatisch.

## Was übernimmt Kubernetes und was nicht? {#kubernetes-boundary}

Kubernetes orchestriert gewünschte Zustände, Scheduling, Service Discovery, Rollouts und
Selbstheilung von Workloads. Es entscheidet nicht automatisch über gute Anwendungsarchitektur,
Datenkonsistenz, Berechtigungsmodell, Kosten oder sinnvolle Betriebsziele.

## Wann ist Serverless eine gute Wahl? {#serverless}

Bei ereignisgetriebenen, schwankenden oder kurzlebigen Workloads, wenn geringe Betriebsarbeit und
schnelle Skalierung wichtiger sind als vollständige Laufzeitkontrolle. Grenzen bei Laufzeit,
Latenz, Portabilität und Kostenprofil müssen zum Use Case passen.

## Wie entscheide ich Build versus Buy? {#build-vs-buy}

Vergleiche strategische Differenzierung, Time-to-Value, Gesamtbetriebskosten, Integrationsaufwand,
Compliance, Exit-Möglichkeiten und interne Fähigkeiten. Standardfähigkeiten werden eher gekauft;
echte Differenzierung und besondere Kontrollanforderungen können Eigenentwicklung rechtfertigen.

## Wie erkläre ich eine Architekturentscheidung einem nichttechnischen Stakeholder? {#explain-architecture}

Beginne mit Ziel und Risiko, nicht mit Komponenten. Zeige zwei oder drei echte Optionen, ihre
Auswirkung auf Zeit, Kosten, Kontrolle und Betrieb und gib eine klare Empfehlung mit den wichtigsten
Annahmen.

## Wie gehe ich mit unklaren Kundenanforderungen um? {#ambiguous-requirements}

Ziel, Nutzer und Entscheidung klären, Annahmen sichtbar machen, Risiken priorisieren und mit einem
kleinen überprüfbaren Artefakt lernen. Ein Workshop endet mit dokumentierten Entscheidungen,
Verantwortlichen und offenen Punkten, nicht nur mit Diskussion.

## Wie reagiere ich auf eine technische Frage, deren Antwort ich nicht weiß? {#unknown-answer}

Keine Sicherheit vortäuschen. Grenzen offen benennen, relevante Annahmen strukturieren, eine
plausible Hypothese formulieren und erklären, wie du sie schnell und risikoarm verifizieren
würdest. Seniorität zeigt sich auch im Umgang mit Unsicherheit.

## Wie beschreibe ich einen Fehler oder ein gescheitertes Projekt? {#failure-story}

Nenne deinen eigenen Anteil, die frühesten übersehenen Signale, die konkrete Korrektur und welche
Arbeitsweise sich danach dauerhaft geändert hat. Vermeide Schuldzuweisung und eine Geschichte, in
der du angeblich von Anfang an recht hattest.

## Wie zeige ich glaubwürdig, dass ich Junioren befähigen kann? {#mentoring}

Mit einem konkreten Beispiel: Ziel und Kontext erklären, Arbeit in überprüfbare Schritte schneiden,
früh Feedback geben, Entscheidungslogik sichtbar machen und Verantwortung schrittweise übertragen.
Der Erfolg ist selbstständigeres Arbeiten, nicht dauerhafte Abhängigkeit von dir.

## Wie leite ich einen technischen Kundenworkshop? {#technical-workshop}

Vorab Ziel, Entscheidungen, Teilnehmer und Inputs festlegen. Im Workshop Problem und Kriterien
gemeinsam bestätigen, Optionen strukturiert bewerten und Konflikte sichtbar machen. Danach folgen
Entscheidungsprotokoll, Verantwortliche, Termine und offene Evidenz.

## Was gehört in ein Cloud Opportunity Assessment? {#cloud-opportunity-assessment}

Anwendungsportfolio, Geschäftswert, technische Eignung, Abhängigkeiten, Daten- und
Regulierungsanforderungen, Fähigkeiten, Kostenbaseline und Migrationsrisiken. Das Ergebnis ist eine
priorisierte Roadmap mit begründeten Wellen, nicht nur ein Ampel-Sheet.

## Wie beantworte ich „Warum BearingPoint?“ ohne Floskeln? {#why-bearingpoint}

Verbinde die Rolle mit konkreten Merkmalen: Management- und Technologieberatung, End-to-End-
Transformation von Strategie bis Betrieb, die veröffentlichte Cloud-Praxis und die Aufgabe,
GenAI-Lösungen sowie das Serviceportfolio mitzugestalten. Ergänze, welche eigene Projekterfahrung
genau dort anschließt.

## Welche Rückfragen zeigen Seniorität am Ende des Interviews? {#candidate-questions}

Frage nach typischen Projektphasen, eigener Verantwortung im ersten Halbjahr, Balance zwischen
Advisory und Umsetzung, Architektur- und Delivery-Standards, Erfolgsmaßen, Teamzuschnitt,
Entscheidungsspielraum sowie den aktuell schwierigsten Cloud- oder GenAI-Problemen der Kunden.
