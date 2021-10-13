# How to save templates in repository

prerequisits:
- Have Templo installed in your computer.

At the beginning of the process you need to have a structure of files and folders to save.

For example:

````
.
+-- Articles
|   +-- article1.md
|   +-- article2.md
+-- Scripts
|   +-- script.py
+-- index.html
````

Suppose you usually needs that structure to develop something.

You can save that structure as a template to use any time.

Look this...

````console
$ tp save .
...
````
> Note: I'm currently working in the same folder that I want to save. 

````
Template name: articles-temp
Repository (main): main
Template description: A template for generate articles and python scripts + index.html
Template was saved successfully.
Done in 15.33ms
````

What happened here?

Templo creates a template based on structure path that you inform as an input in command above and save it in main repository.

You can check if it really happens.

````console
$ tp repo main
...
````

| name | created-at |
|-----|--------|
| articles-temp | 2021-10-13 17:23:24 |

View the template data...

````console
$ tp view main/articles-temp
> articles-temp
A template for generate articles and python scripts + index.html

[CREATED AT]
    2021-10-13 17:23:24

[PATHS]
    Articles
    Articles/article1.md
    Articles/article2.md
    index.html
    Scripts
    Scripts/script.py
````
