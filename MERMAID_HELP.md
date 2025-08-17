## Special Instructions to follow when generating Mermaid charts in md files.
* The mermaid charts are being rendered in VS Code using the Markdown Preview Mermaid extension which only supports mermaid syntax up to 11.6.0. Make sure the charts you generate are compatable with that version.
* There a few other incompatabilities so please take care not to use the following:

  1. Use ```packet-beta``` as the diagram type for packet diagrams
  1. Size annotations like ```+10: First Name``` are not supported in packet-beta diagrams so use the range syntax instead: ```0-32: First Name```