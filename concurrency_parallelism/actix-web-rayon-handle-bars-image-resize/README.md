# Introduction
The sample tries to take in an image with x, y dimensions via HTTP POST method and make two variants of image as below.
1. A picture with (400, 400)px.
2. A thumbnail with (100, 100)px.

## During the course, the following questions are answered.

1. How does the buffer transmit between the HTTP request and the rayon parallel tasks.
2. Whats the best way to make the large buffer reach client ?

# Features
1. Ability upload a picture to server.
2. Ability to see two re-sized variants after upload complete.
3. Ability to report the time taken for the operation along with picture.
4. Picture should start showing when done.
5. Only png, jpeg -> .png is supported.
