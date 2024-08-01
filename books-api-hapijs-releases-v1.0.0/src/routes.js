const { deleteBookById } = require("./controller/deleteBook");
const { getAllBooks, getBookById } = require("./controller/getBook");
const { saveBook } = require("./controller/saveBook");
const { updateBook } = require("./controller/updateBook");

const routes = [
    {
        method: "POST",
        path: "/books",
        handler: saveBook

    },
    {
        method: "GET",
        path: "/books",
        handler: getAllBooks

    },
    {
        method: "GET",
        path: "/books/{bookId}",
        handler: getBookById
    },
    {
        method: "PUT",
        path: "/books/{bookId}",
        handler: updateBook
    },
    {
        method: "DELETE",
        path: "/books/{bookId}",
        handler: deleteBookById
    }
];

module.exports = routes;
