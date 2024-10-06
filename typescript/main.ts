import Quaternion from './src'

const q1 = new Quaternion(1, 1, 1, 1)
const q2 = new Quaternion(0, 1, 0, 1)
console.log('q1:', q1.toString())
console.log('q1 to vector:', q1.toVector3())
console.log('q1 + q2 =', Quaternion.add(q1, q2))
console.log('q1 * q2 =', Quaternion.multiply(q1, q2))
console.log('q1 - q2 =', Quaternion.subtract(q1, q2))
console.log('q1 / q2 =', Quaternion.divide(q1, q2))
console.log('Conjugate of q1:', q1.conjugate())
console.log('Sign number of q1:', q1.sgn())
console.log('Absolute value of q1:', q1.abs())
console.log('Inverse of q1:', q1.inverse())
console.log('Rotating vector [1, 0, 0] by q1:', q1.rotateVector([1, 0, 0]))
console.log('Euler angles of q1:', q1.toEulerAngles())

