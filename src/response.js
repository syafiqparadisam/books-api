class response {
    constructor(status) {
        this.status = status;
    }

    setMessage(message) {
        this.message = message;
        return this;
    }

    setData(data) {
        this.data = data;
        return this;
    }
}

module.exports = response;
