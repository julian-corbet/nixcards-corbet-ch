# Warum sind Workload-Identitäten besser als langlebige Zugangsschlüssel?

Sie ermöglichen kurzlebige, automatisch ausgestellte Berechtigungen für einen konkreten Workload.
Dadurch entfallen verteilte statische Secrets, Rotation wird einfacher und Zugriffe lassen sich
besser einem Laufzeitkontext zuordnen.
