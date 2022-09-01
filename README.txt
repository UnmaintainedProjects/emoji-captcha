Emoji CAPTCHA
-------------

A CAPTCHA implementation based on identifying emojis. This was exclusively
developed to be used in Telegram, but might be used anywhere else possible.

Brief Description
-----------------

1. The client sends an HTTP request to the server.

2. The server returns a generated photo that has 6 emojis inside it, along with:

- A custom header called `x-emojis` that includes the code points of the six
  emojis in the photo plus 2 random ones in hex, shuffled, splitted by ";",
  and the parts of each of them splitted by "-".

- A custom header called `x-correct-emojis` that only includes the code points
  of the 6 emojis in the photo.

3. The client displays the photo returned by the server, along with the emojis
   provided in the `x-emojis` in a way the user can pick them.

4. The client compares the first six emojis that the user picked with `x-correct-emojis`.

Credits
-------

This wouldn't be possible without the inspirations from the CAPTCHA system at @PyrogramLounge.
