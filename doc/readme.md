### How to use this project

This project works base on 2 folder

1. base
2. content

When you have a static website, you have 2 important parts
That you need to make. the _base_ html part and the _content_
Part. base html part is the actual html file (style of your blog)
That it is static and it will not changes. And the _content_ part
That is not static and will changes by each content that you post.

With that said, imagine you have these structure for your website

---------
Base (folder)
  index.html
  blog.html
Content (folder)
  hello.con
  another.con
  nice.con
staticy (executable)
---------

With this structure we have:
Base/index.html: your main entry of your blog
Base/blog.html: your base html file for each pages
Content/hello.con: A simple file that include your content
Content/another.con: ~
Content/nice.con: ~


## Compile and make website
For compiling all those files into a static website.
The only thing you need is to just run the binary
```
staticy
```
That's it! now you have a dist folder that contains all
of your contents as a website. you can run index.html and
see the result.
