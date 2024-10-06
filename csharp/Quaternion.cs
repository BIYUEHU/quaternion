using System;

namespace QuaternionCs
{
    public interface IQuaternion
    {
        Vector3 ToEulerAngles();
        Vector3 ToVector3();
        Vector3 RotateVector(Vector3 v);
        Vector3 MultiplyVector(Vector3 v);
        string ToString();
        Quaternion Conjugate();
        double Abs();
        Quaternion Sgn();
        Quaternion Inverse();
        double Arg();
    }

    public class Quaternion : IQuaternion
    {
        public static Quaternion FromAxisAngle(Vector3 axis, double angle)
        {
            double halfAngle = angle / 2;
            double sinHalfAngle = Math.Sin(halfAngle);
            return new Quaternion(
                Math.Cos(halfAngle),
                axis.X * sinHalfAngle,
                axis.Y * sinHalfAngle,
                axis.Z * sinHalfAngle
            );
        }

        public static Quaternion Add(Quaternion q1, Quaternion q2)
        {
            return new Quaternion(q1.W + q2.W, q1.I + q2.I, q1.J + q2.J, q1.K + q2.K);
        }

        public static Quaternion Subtract(Quaternion q1, Quaternion q2)
        {
            return new Quaternion(q1.W - q2.W, q1.I - q2.I, q1.J - q2.J, q1.K - q2.K);
        }

        public static Quaternion Multiply(Quaternion q1, Quaternion q2)
        {
            return new Quaternion(
                q1.W * q2.W - q1.I * q2.I - q1.J * q2.J - q1.K * q2.K,
                q1.W * q2.I + q1.I * q2.W + q1.J * q2.K - q1.K * q2.J,
                q1.W * q2.J - q1.I * q2.K + q1.J * q2.W + q1.K * q2.I,
                q1.W * q2.K + q1.I * q2.J - q1.J * q2.I + q1.K * q2.W
            );
        }

        public static Quaternion Divide(Quaternion q1, Quaternion q2)
        {
            return Multiply(q1, q2.Conjugate());
        }

        public static Double ScalarProduct(Quaternion q1, Quaternion q2)
        {
            return q1.W * q2.W + q1.I * q2.I + q1.J * q2.J + q1.K * q2.K;
        }

        public static Quaternion OuterProduct(Quaternion q1, Quaternion q2)
        {
            return new Quaternion(
              q1.W * q2.W - q1.I * q2.I - q1.J * q2.J - q1.K * q2.K,
              q1.W * q2.I + q1.I * q2.W + q1.J * q2.K - q1.K * q2.J,
              q1.W * q2.J - q1.I * q2.K + q1.J * q2.W + q1.K * q2.I,
              q1.W * q2.K + q1.I * q2.J - q1.J * q2.I + q1.K * q2.W
            );
        }

        public static Quaternion EvenProduct(Quaternion q1, Quaternion q2)
        {
            return new Quaternion(
              q1.W * q2.W - q1.I * q2.I - q1.J * q2.J - q1.K * q2.K,
              q1.W * q2.I + q1.I * q2.W - q1.J * q2.K + q1.K * q2.J,
              q1.W * q2.J + q1.I * q2.K + q1.J * q2.W - q1.K * q2.I,
              q1.W * q2.K - q1.I * q2.J + q1.J * q2.I + q1.K * q2.W
            );
        }

        public static Quaternion CrossProduct(Quaternion q1, Quaternion q2)
        {
            return new Quaternion(
              q1.W * q2.W - q1.I * q2.I - q1.J * q2.J - q1.K * q2.K,
              q1.W * q2.I + q1.I * q2.W + q1.J * q2.K - q1.K * q2.J,
              q1.W * q2.J - q1.I * q2.K + q1.J * q2.W + q1.K * q2.I,
              q1.W * q2.K + q1.I * q2.J - q1.J * q2.I + q1.K * q2.W
            );
        }

        public double W { get; }
        public double I { get; }
        public double J { get; }
        public double K { get; }

        public Quaternion(double w = 1.0, double i = 0.0, double j = 0.0, double k = 0.0)
        {
            W = w;
            I = i;
            J = j;
            K = k;
        }

        public Vector3 RotateVector(Vector3 v)
        {
            Quaternion vq = new Quaternion(0, v.X, v.Y, v.Z);
            Quaternion resultQ = Multiply(vq, Conjugate());
            return new Vector3(resultQ.I, resultQ.J, resultQ.K);
        }

        public Vector3 MultiplyVector(Vector3 v)
        {
            Quaternion qv = new Quaternion(0.0, v.X, v.Y, v.Z);
            Quaternion resultQ = Quaternion.Multiply(qv, Conjugate());
            return new Vector3(resultQ.I, resultQ.J, resultQ.K);
        }

        public Vector3 ToEulerAngles()
        {
            double roll = Math.Atan2(2 * (W * I + J * K), 1 - 2 * (I * I + J * J));
            double pitch = Math.Asin(2 * (W * J - K * I));
            double yaw = Math.Atan2(2 * (W * K + I * J), 1 - 2 * (J * J + K * K));
            return new Vector3(roll, pitch, yaw);
        }

        public Vector3 ToVector3()
        {
            return new Vector3(I, J, K);
        }

        public override string ToString()
        {
            string getSign(double num) => num >= 0 ? "+" : "-";
            return $"{W}{getSign(I)}{Math.Abs(I)}i{getSign(J)}{Math.Abs(J)}j{getSign(K)}{Math.Abs(K)}k";
        }

        public Quaternion Conjugate()
        {
            return new Quaternion(W, -I, -J, -K);
        }

        public double Abs()
        {
            return Math.Sqrt(W * W + I * I + J * J + K * K);
        }

        public Quaternion Sgn()
        {
            double abs = Abs();
            return new Quaternion(W / abs, I / abs, J / abs, K / abs);
        }

        public Quaternion Inverse()
        {
            double abs2 = Abs() * Abs();
            return new Quaternion(W / abs2, -I / abs2, -J / abs2, -K / abs2);
        }

        public double Arg()
        {
            return Math.Acos(W / Abs());
        }
    }
}
