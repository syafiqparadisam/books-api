# Books API HapIJS

This project is built from hapijs and i got task from dicoding to create project using hapiJS to complete course and get certificate

## Tech stack
1. **NodeJS**: RUNTIME JAVASCRIPT
2. **HapiJS**: API
3. **JSON server**: DATABASE 


## Feature

## 1. Save Book

You can save book by call this api, and send payload example like this

_METHOD: POST_

_PATH: /books_

_PAYLOAD:_

```
{
 "name": "YOURNAME",
 "year": 2024,
 "author": "AUTHOR",
 "summary": "SUMMARY",
 "publisher": "PUBLISHER",
 "pageCount": 20,
 "readPage": 0,
 "reading": true
}
```

## 2. Get All books
Get all books after you save your book

_METHOD: GET_

_PATH: /books_

_OPTIONAL QUERY:_

_?name=_ GET all books by name

_?reading=_ GET all books by is already read

_?finished=_ GET all books by is finished

*Note* : Query can't combined, only use one of query above

## 3. Get book by id
GET book by id

_METHOD: GET_

_PATH: /books/{bookId}_

## 4. Update book
UPDATE book

_METHOD: PUT_

_PATH: /books/{bookId}_

_PAYLOAD:_


```
{
 "name": "YOURNAME",
 "year": 2024,
 "author": "AUTHOR",
 "summary": "SUMMARY",
 "publisher": "PUBLISHER",
 "pageCount": 20,
 "readPage": 0,
 "reading": true
}
```

## 5. Delete book by id
DELETE book by id

_METHOD: DELETE_

_PATH: /books/{bookId}_

## How to use this project

### 1. Clone this repo
```
git clone https://github.com/syafiqparadisam/books-api-hapijs.git
```
or you may use other method to clone this repo

### 2. Run this API
```
npm start
```

## Contact and Support

If you encounter any issues or have questions, please contact our email or dm me on instagram, i already provide on my bio.

Please give me star if you like and follow my account

Thank you!