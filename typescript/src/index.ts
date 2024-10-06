export type Vector3 = [number, number, number]

interface Quaternion {
  readonly w: number
  readonly i: number
  readonly j: number
  readonly k: number
  toEulerAngles(): Vector3
  toVector3(): Vector3
  rotateVector(v: Vector3): Vector3
  multiplyVector(v: Vector3): Vector3
  toString(): string
  conjugate(): Quaternion
  abs(): number
  sgn(): Quaternion
  inverse(): Quaternion
  arg(): number
}

class Quaternion implements Quaternion {
  public static fromAxisAngle(axis: Vector3, angle: number) {
    const [i, j, k] = axis
    const halfAngle = angle / 2
    const sinHalfAngle = Math.sin(halfAngle)
    return new Quaternion(Math.cos(halfAngle), i * sinHalfAngle, j * sinHalfAngle, k * sinHalfAngle)
  }

  public static add(q1: Quaternion, q2: Quaternion) {
    return new Quaternion(q1.w + q2.w, q1.i + q2.i, q1.j + q2.j, q1.k + q2.k)
  }

  public static subtract(q1: Quaternion, q2: Quaternion) {
    return new Quaternion(q1.w - q2.w, q1.i - q2.i, q1.j - q2.j, q1.k - q2.k)
  }

  public static multiply(q1: Quaternion, q2: Quaternion) {
    return new Quaternion(
      q1.w * q2.w - q1.i * q2.i - q1.j * q2.j - q1.k * q2.k,
      q1.w * q2.i + q1.i * q2.w + q1.j * q2.k - q1.k * q2.j,
      q1.w * q2.j - q1.i * q2.k + q1.j * q2.w + q1.k * q2.i,
      q1.w * q2.k + q1.i * q2.j - q1.j * q2.i + q1.k * q2.w
    )
  }

  public static divide(q1: Quaternion, q2: Quaternion) {
    return Quaternion.multiply(q1, q2.conjugate())
  }

  public static scalarProduct(q1: Quaternion, q2: Quaternion) {
    return q1.w * q2.w + q1.i * q2.i + q1.j * q2.j + q1.k * q2.k
    // return Quaternion.divide(
    //   Quaternion.add(Quaternion.multiply(q1.conjugate(), q2), Quaternion.multiply(q1, q2.conjugate())),
    //   new Quaternion(2)
    // )
  }

  public static outerProduct(q1: Quaternion, q2: Quaternion) {
    return new Quaternion(
      q1.w * q2.w - q1.i * q2.i - q1.j * q2.j - q1.k * q2.k,
      q1.w * q2.i + q1.i * q2.w + q1.j * q2.k - q1.k * q2.j,
      q1.w * q2.j - q1.i * q2.k + q1.j * q2.w + q1.k * q2.i,
      q1.w * q2.k + q1.i * q2.j - q1.j * q2.i + q1.k * q2.w
    )

  }

  public static evenProduct(q1: Quaternion, q2: Quaternion) {
    return new Quaternion(
      q1.w * q2.w - q1.i * q2.i - q1.j * q2.j - q1.k * q2.k,
      q1.w * q2.i + q1.i * q2.w - q1.j * q2.k + q1.k * q2.j,
      q1.w * q2.j + q1.i * q2.k + q1.j * q2.w - q1.k * q2.i,
      q1.w * q2.k - q1.i * q2.j + q1.j * q2.i + q1.k * q2.w
    )
  }

  public static crossProduct(q1: Quaternion, q2: Quaternion) {
    return new Quaternion(
      q1.w * q2.w - q1.i * q2.i - q1.j * q2.j - q1.k * q2.k,
      q1.w * q2.i + q1.i * q2.w + q1.j * q2.k - q1.k * q2.j,
      q1.w * q2.j - q1.i * q2.k + q1.j * q2.w + q1.k * q2.i,
      q1.w * q2.k + q1.i * q2.j - q1.j * q2.i + q1.k * q2.w
    )
  }

  public constructor(
    public readonly w = 1,
    public readonly i = 0,
    public readonly j = 0,
    public readonly k = 0
  ) {}

  public rotateVector(v: Vector3): Vector3 {
    const [x, y, z] = v
    const vq = new Quaternion(0, x, y, z)
    const resultQ = Quaternion.multiply(vq, this.conjugate())
    return [resultQ.i, resultQ.j, resultQ.k]
  }

  public multiplyVector(v: Vector3): Vector3 {
    const [x, y, z] = v
    const qv = new Quaternion(0, x, y, z)
    const resultQ = Quaternion.multiply(qv, this.conjugate())
    return [resultQ.i, resultQ.j, resultQ.k]
  }

  public toEulerAngles(): Vector3 {
    const [w, i, j, k] = [this.w, this.i, this.j, this.k]
    const roll = Math.atan2(2 * (w * i + j * k), 1 - 2 * (i ** 2 + j ** 2))
    const pitch = Math.asin(2 * (w * j - k * i))
    const yaw = Math.atan2(2 * (w * k + i * j), 1 - 2 * (j ** 2 + k ** 2))
    return [roll, pitch, yaw]
  }

  public toVector3(): Vector3 {
    return [this.i, this.j, this.k]
  }

  public toString() {
    const getSign = (num: number) => (num >= 0 ? '+' : '-')
    return `${this.w}${getSign(this.i)}${Math.abs(this.i)}i${getSign(this.j)}${Math.abs(this.j)}j${getSign(this.k)}${Math.abs(this.k)}k`
  }

  public conjugate() {
    return new Quaternion(this.w, -this.i, -this.j, -this.k)
  }

  public abs() {
    return Math.sqrt(this.w ** 2 + this.i ** 2 + this.j ** 2 + this.k ** 2)
  }

  public sgn() {
    const abs = this.abs()
    return new Quaternion(this.w / abs, this.i / abs, this.j / abs, this.k / abs)
  }

  public inverse() {
    const abs2 = this.abs() ** 2
    return new Quaternion(this.w / abs2, -this.i / abs2, -this.j / abs2, -this.k / abs2)
  }

  public arg() {
    return Math.acos(this.w / this.abs())
  }
}

export default Quaternion
