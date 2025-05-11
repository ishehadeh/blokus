// cgtjs/Board.ts
function deltaSwap(board, mask, delta) {
  const x = (board ^ board >> delta) & mask;
  return board ^ x << delta ^ x;
}

class BitBoard {
  #bits;
  #width;
  #height;
  #maskRow = 0n;
  #maskCol = 0n;
  get width() {
    return this.#width;
  }
  get height() {
    return this.#height;
  }
  get maskRow() {
    return this.#maskRow;
  }
  get maskCol() {
    return this.#maskCol;
  }
  get bits() {
    return BigInt.asUintN(Number(this.width * this.height), this.#bits);
  }
  set bits(bits) {
    this.#bits = BigInt.asUintN(Number(this.width * this.height), bits);
  }
  constructor(w, h, bits = 0n) {
    this.#width = w;
    this.#height = h;
    this.#bits = BigInt.asUintN(Number(w * h), bits);
    this.#maskCol = BigInt.asUintN(Number(w * h), 0n);
    this.#maskRow = BigInt.asUintN(Number(w * h), 0n);
    for (let i = 0n;i < this.width; ++i) {
      this.#maskRow |= 1n << i;
    }
    for (let i = 0n;i < this.height; ++i) {
      this.#maskCol |= 1n << i * this.width;
    }
  }
  resize(w, h) {
    const newBoard = new BitBoard(w, h);
    const rowMask = this.width < w ? this.maskRow : newBoard.maskRow;
    const copyWidth = this.width < w ? this.width : w;
    const copyHeight = this.height > h ? h : this.height;
    for (let i = 0n;i < copyHeight; ++i) {
      newBoard.#bits |= (this.#bits & rowMask << i * copyWidth) >> i * copyWidth << i * w;
    }
    return newBoard;
  }
  clone() {
    return new BitBoard(this.width, this.height, this.bits);
  }
  translateInPlace(x, y) {
    while (x < 0) {
      this.#bits &= ~this.maskCol;
      this.#bits >>= 1n;
      x += 1n;
    }
    while (x > 0) {
      this.#bits &= ~(this.maskCol << this.width - 1n);
      this.#bits <<= 1n;
      x -= 1n;
    }
    while (y < 0) {
      this.#bits >>= this.width;
      y += 1n;
    }
    while (y > 0) {
      this.#bits &= ~(this.maskRow << (this.height - 1n) * this.width);
      this.#bits <<= this.height;
      y -= 1n;
    }
  }
  translate(x, y) {
    const tr = new BitBoard(this.width, this.height, this.bits);
    tr.translateInPlace(x, y);
    return tr;
  }
  get(x, y) {
    return (this.#bits & 1n << y * this.width + x) > 0;
  }
  set(x, y) {
    this.#bits |= 1n << y * this.width + x;
  }
  clear(x, y) {
    this.#bits &= ~(1n << y * this.width + x);
  }
  flipVerticalInPlace() {
    let newBits = BigInt.asUintN(Number(this.width * this.height), 0n);
    for (let i = 0n;i < this.height; ++i) {
      const rowTopOffset = i * this.width;
      const rowBottomOffset = (this.height - i - 1n) * this.width;
      newBits |= (this.maskRow & this.#bits >> rowTopOffset) << rowBottomOffset;
      newBits |= (this.maskRow & this.#bits >> rowBottomOffset) << rowTopOffset;
    }
    this.#bits = newBits;
  }
  transpose() {
    const newBoard = new BitBoard(this.height, this.width);
    for (let x = 0n;x < this.width; ++x) {
      for (let y = 0n;y < this.height; ++y) {
        if (this.get(x, y)) {
          newBoard.set(y, x);
        }
      }
    }
    return newBoard;
  }
  flipHorizontalInPlace() {
    let newBits = BigInt.asUintN(Number(this.width * this.height), 0n);
    for (let i = 0n;i < (this.width + 1n) / 2n; ++i) {
      const colLeftOffset = i;
      const colRightOffset = this.width - i - 1n;
      newBits |= (this.maskCol & this.#bits >> colLeftOffset) << colRightOffset;
      newBits |= (this.maskCol & this.#bits >> colRightOffset) << colLeftOffset;
    }
    this.#bits = newBits;
  }
  toString() {
    let str = "";
    for (let y = 0n;y < this.height; ++y) {
      for (let x = 0n;x < this.width; ++x) {
        str += this.get(x, y) ? "1" : "0";
      }
      str += "\n";
    }
    return str;
  }
  rotateClockwise() {
    const tr = this.transpose();
    tr.flipHorizontalInPlace();
    return tr;
  }
  *iterSet() {
    for (let i = 0n;i < this.width * this.height; ++i) {
      if ((this.#bits & 1n << i) > 0n) {
        yield [i % this.width, i / this.width];
      }
    }
  }
  *iterClear() {
    for (let i = 0n;i < this.width * this.height; ++i) {
      if ((this.#bits & 1n << i) == 0n) {
        yield [i % this.width, i / this.width];
      }
    }
  }
}

// cgtjs/game/Blokus.ts
var TileState;
((TileState2) => {
  TileState2["Interior"] = "interior";
  TileState2["Corner"] = "corner";
  TileState2["Side"] = "side";
  TileState2["Empty"] = "empty";
})(TileState ||= {});

class Blokus {
  #side;
  #corner;
  #interior;
  get width() {
    return this.#side.width;
  }
  get height() {
    return this.#side.height;
  }
  constructor(side, corner, interior) {
    this.#side = side;
    this.#corner = corner;
    this.#interior = interior;
  }
  static empty(w, h) {
    return new Blokus(new BitBoard(w, h), new BitBoard(w, h), new BitBoard(w, h));
  }
  static fromString(str) {
    const rowsTrimmed = str.split("\n").map((r) => r.trim()).filter((r) => r.length !== 0);
    const w = BigInt(Math.max(...rowsTrimmed.map((r) => r.length)));
    const h = BigInt(rowsTrimmed.length);
    const game = Blokus.empty(w, h);
    let y = 0n;
    for (const row of rowsTrimmed) {
      let x = 0n;
      for (const cell of row) {
        switch (cell) {
          case ".":
            game.set(x, y, "empty" /* Empty */);
            break;
          case "i":
            game.set(x, y, "interior" /* Interior */);
            break;
          case "c":
            game.set(x, y, "corner" /* Corner */);
            break;
          case "s":
            game.set(x, y, "side" /* Side */);
            break;
          default:
            throw TypeError("unexpected character");
        }
        x += 1n;
      }
      y += 1n;
    }
    return game;
  }
  isEqualTo(other) {
    return other.width === this.width && other.height === this.height && other.#corner.bits === this.#corner.bits && other.#side.bits === this.#side.bits && other.#interior.bits === this.#interior.bits;
  }
  toStringBoard() {
    let boardStr = "";
    for (let y = 0n;y < this.height; ++y) {
      for (let x = 0n;x < this.width; ++x) {
        switch (this.get(x, y)) {
          case "side" /* Side */:
            boardStr += "s";
            break;
          case "corner" /* Corner */:
            boardStr += "c";
            break;
          case "interior" /* Interior */:
            boardStr += "i";
            break;
          case "empty" /* Empty */:
            boardStr += ".";
            break;
        }
      }
      boardStr += "\n";
    }
    return boardStr;
  }
  get(x, y) {
    if (this.#side.get(x, y))
      return "side" /* Side */;
    if (this.#corner.get(x, y))
      return "corner" /* Corner */;
    if (this.#interior.get(x, y))
      return "interior" /* Interior */;
    return "empty" /* Empty */;
  }
  set(x, y, state) {
    switch (state) {
      case "interior" /* Interior */:
        this.#side.clear(x, y);
        this.#corner.clear(x, y);
        this.#interior.set(x, y);
        break;
      case "side" /* Side */:
        this.#side.set(x, y);
        this.#corner.clear(x, y);
        this.#interior.clear(x, y);
        break;
      case "corner" /* Corner */:
        this.#side.clear(x, y);
        this.#corner.set(x, y);
        this.#interior.clear(x, y);
        break;
      case "empty" /* Empty */:
        this.#side.clear(x, y);
        this.#corner.clear(x, y);
        this.#interior.clear(x, y);
        break;
      default:
        throw new TypeError(`invalid state: "${state}"`);
    }
  }
  clone() {
    return new Blokus(this.#side.clone(), this.#corner.clone(), this.#interior.clone());
  }
  toString() {
    return `Blokus(${this.width}, ${this.height}, ${this.#side.bits}, ${this.#corner.bits}, ${this.#interior.bits})`;
  }
  countInterior() {
    let bits = this.#interior.bits;
    let count = 0n;
    while (bits > 0n) {
      count += bits & 1n;
      bits >>= 1n;
    }
    return count;
  }
  resize(w, h) {
    return new Blokus(this.#side.resize(w, h), this.#corner.resize(w, h), this.#interior.resize(w, h));
  }
  rotateClockwise() {
    return new Blokus(this.#side.rotateClockwise(), this.#corner.rotateClockwise(), this.#interior.rotateClockwise());
  }
  translateInPlace(x, y) {
    this.#side.translateInPlace(x, y);
    this.#corner.translateInPlace(x, y);
    this.#interior.translateInPlace(x, y);
  }
  assertValid() {
    console.assert((this.#corner.bits & this.#interior.bits) === 0n);
    console.assert((this.#corner.bits & this.#side.bits) === 0n);
    console.assert((this.#side.bits & this.#interior.bits) === 0n);
  }
  tryPlacePolyomino(x, y, polyomino, polyX, polyY) {
    const trPolyomino = polyomino.resize(this.#corner.width, this.#corner.height);
    trPolyomino.translateInPlace(x - polyX, y - polyY);
    if (trPolyomino.countInterior() != polyomino.countInterior()) {
      return false;
    }
    if ((this.#interior.bits & (trPolyomino.#interior.bits | trPolyomino.#side.bits)) != 0n) {
      return false;
    }
    this.#interior.bits |= trPolyomino.#interior.bits;
    this.#side.bits |= trPolyomino.#side.bits & ~this.#interior.bits;
    this.#corner.bits |= trPolyomino.#corner.bits & ~(this.#interior.bits | this.#side.bits);
    this.#corner.bits &= ~(trPolyomino.#interior.bits | trPolyomino.#side.bits);
    this.#side.bits &= ~trPolyomino.#interior.bits;
    return true;
  }
  *moves(polyominos) {
    for (const polyomino of polyominos) {
      let currentPoly = polyomino;
      for (let rotation = 0;rotation < 4; rotation++) {
        if (rotation > 0) {
          currentPoly = currentPoly.rotateClockwise();
        }
        for (const [boardX, boardY] of this.#corner.iterSet()) {
          for (const [polyX, polyY] of currentPoly.#corner.iterSet()) {
            const newBoard = this.clone();
            if (newBoard.tryPlacePolyomino(boardX, boardY, currentPoly, polyX + 1n, polyY + 1n)) {
              yield newBoard;
            }
          }
        }
      }
    }
  }
}
export {
  deltaSwap,
  TileState,
  Blokus,
  BitBoard
};
