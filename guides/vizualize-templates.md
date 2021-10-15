# How to vizualize templates

prerequisits:
- Have Templo installed in your computer
- Have a template to vizualize

To vizualize template we will use the **view** method.

Look this...

````console
$ tp view main/articles-temp
````

All template information will be displayed.

````
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

You can display each template info as well.

````console
$ tp view main/articles-temp --created-at
2021-10-13 17:23:24
````

````console
$ tp view main/articles-temp --paths
Articles
Articles/article1.md
Articles/article2.md
index.html
Scripts
Scripts/script.py
````

## Vizualizing files content

````console
$ tp view main/articles-temp file=index.html
````

The template file content will be displayed.

...