const response = require("../response");
const { books } = require("../data/books");

const updateBook = (request, h) => {
    const bookId = request.params.bookId;
    const { name, year, author, summary, publisher, pageCount, readPage, reading } = request.payload;
    if (!name) {
        return h.response(new response("fail").setMessage("Gagal memperbarui buku. Mohon isi nama buku")).code(400);
    }
    if (readPage > pageCount) {
        return h.response(new response("fail").setMessage("Gagal memperbarui buku. readPage tidak boleh lebih besar dari pageCount")).code(400);
    }

    const foundBook = books.find(book => book.id == bookId);
    if (!foundBook) {
        return h.response(new response("fail").setMessage("Gagal memperbarui buku. Id tidak ditemukan")).code(404);
    }
    foundBook.name = name;
    foundBook.year = year;
    foundBook.author = author;
    foundBook.summary = summary;
    foundBook.publisher = publisher;
    foundBook.pageCount = pageCount;
    foundBook.readPage = readPage;
    foundBook.reading = reading;

    return h.response(new response("success").setMessage("Buku berhasil diperbarui")).code(200);
};

module.exports = {
    updateBook
};
