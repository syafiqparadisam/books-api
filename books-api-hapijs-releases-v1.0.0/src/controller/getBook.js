const { books } = require("../data/books");
const response = require("../response");

const getAllBooks = (request, h) => {
    const name = request.query.name;
    const reading = request.query.reading;
    const finished = request.query.finished;
    if (name) {
        const foundBook = books.filter((book) => {
            const regex = new RegExp(name, "i");
            return regex.test(book.name);
        });
        return h.response(new response("success").setData({ books: foundBook.length == 0 ? [] : foundBook })).code(200);
    }

    if (reading) {
        const foundBook = reading == "0" ? books.filter((v) => v.reading == false) : books.filter((v) => v.reading == true);
        return h.response(new response("success").setData({ books: foundBook.length == 0 ? [] : foundBook })).code(200);
    }

    if (finished) {
        const foundBook = finished == "0" ? books.filter((v) => v.finished == false) : books.filter((v) => v.finished == true);
        return h.response(new response("success").setData({ books: foundBook.length == 0 ? [] : foundBook })).code(200);
    }
    const arrayOfBooksNameIdPublisher = [];
    for (let i = 0; i <= books.length - 1; i++) {
        const obj = {
            id: books[i].id,
            name: books[i].name,
            publisher: books[i].publisher
        };
        arrayOfBooksNameIdPublisher.push(obj);
    }
    return h.response(new response("success").setData({ books: arrayOfBooksNameIdPublisher.length == 0 ? [] : arrayOfBooksNameIdPublisher })).code(200);
};

const getBookById = (request, h) => {
    const bookId = request.params.bookId;

    const foundBook = books.find((book) => book.id == bookId);
    if (!foundBook || foundBook == null) {
        return h.response(new response("fail").setMessage("Buku tidak ditemukan")).code(404);
    }

    return h.response(new response("success").setData({ book: foundBook }));
};

module.exports = {
    getAllBooks,
    getBookById
};
