const { books } = require("../data/books");
const response = require("../response");

const deleteBookById = (request, h) => {
    const bookId = request.params.bookId;

    const foundBook = books.find(book => book.id == bookId);
    if (!foundBook) {
        return h.response(new response("fail").setMessage("Buku gagal dihapus. Id tidak ditemukan")).code(404);
    }
    const index = books.indexOf(foundBook);
    books.splice(index, index + 1);
    return h.response(new response("success").setMessage("Buku berhasil dihapus")).code(200);
};

module.exports = {
    deleteBookById
};
