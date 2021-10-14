# How to get templates from web servers

prerequisits:
- Have Templo installed in your computer

You can get a template from a http adress.

Look this...

````console
$ tp get https://pultzlucas-templo-server.herokuapp.com/templates/deno-server main
````

Templo will connect with the web server using the url.

````
[getting template...]
Enjoy the 'deno-server' template :D
by pultzlucas
Template "deno-server" was saved in "main" repo.
Done in 4.83s
````

You can check if the template was saved.

````console
$ tp repo main
...
````

| name | created-at |
|-----|--------|
| articles-temp | 2021-10-13 17:23:24 |