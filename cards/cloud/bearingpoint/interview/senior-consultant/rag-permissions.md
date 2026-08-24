# Wie setzt man Dokumentberechtigungen in RAG durch?

Die Nutzeridentität und ihre Zugriffsrechte müssen bereits das Retrieval filtern. Erst erlaubte
Chunks dürfen den Prompt erreichen. Ein Filter erst nach der Modellantwort ist zu spät, weil das
Modell den verbotenen Inhalt bereits verarbeitet hat.
