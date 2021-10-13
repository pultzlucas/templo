# How to generate templates

prerequisits:
- Seen how to save templates guide
- Have a template

Generate templates is pretty simple.

Look this...

````console
$ tp gen main/articles-temp .
[creating files and folders...]
dir:  ./Articles
file: ./Articles/article1.md
file: ./Articles/article2.md
file: ./index.html
dir:  ./Scripts
file: ./Scripts/script.py

[writing contents...]
./Articles/article1.md...ok
./Articles/article2.md...ok
./index.html...ok
./Scripts/script.py...ok

Template "articles-temp" was generated.
Done in 51.49ms
````

Now see your current work diretory.

````
.
+-- Articles
|   +-- article1.md
|   +-- article2.md
+-- Scripts
|   +-- script.py
+-- index.html
````

What happened here?

Templo searched the specified template (articles-temp) in a repository (main) and generates the template paths in the specified directory (.).

Go Work!💻


