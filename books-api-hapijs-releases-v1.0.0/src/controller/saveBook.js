const { nanoid } = require("nanoid");
const response = require("../response");
const { books } = require("../data/books");

const saveBook = (request, h) => {
    const { name, year, author, summary, publisher, pageCount, readPage, reading } = request.payload;
    if (!name) {
        return h.response(new response("fail").setMessage("Gagal menambahkan buku. Mohon isi nama buku")).code(400);
    }
    if (readPage > pageCount) {
        return h.response(new response("fail").setMessage("Gagal menambahkan buku. readPage tidak boleh lebih besar dari pageCount")).code(400);
    }
    const id = nanoid(16);
    const data = {
        id,
        name,
        year,
        author,
        summary,
        publisher,
        pageCount,
        readPage,
        finished: pageCount === readPage ? true : false,
        reading,
        insertedAt: new Date().toISOString(),
        updatedAt: new Date().toISOString()
    };

    books.push(data);
    return h.response(new response("success").setMessage("Buku berhasil ditambahkan").setData({ bookId: id })).code(201);
};

module.exports = {
    saveBook
};
