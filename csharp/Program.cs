using System;
/*using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;*/

namespace QuaternionCs
{
    internal class Program
    {
        static void Main(string[] args)
        {
            Quaternion q1 = new Quaternion(1.0, 1.0 , 1.0, 1.0);
            Quaternion q2 = new Quaternion(0.0, 0.0, 0.0, 0.0);
            Vector3 v = new Vector3(1.0, 0.0, 0.0);
            Console.WriteLine($"q1: {q1}");
            Console.WriteLine($"q1 to vector: {q1.ToVector3()}");
            Console.WriteLine($"q1 + q2 = {Quaternion.Add(q1, q2)}");
            Console.WriteLine($"q1 * q2 = {Quaternion.Multiply(q1, q2)}");
            Console.WriteLine($"q1 - q2 = {Quaternion.Subtract(q1, q2)}");
            Console.WriteLine($"q1 / q2 = {Quaternion.Divide(q1, q2)}");
            Console.WriteLine($"Conjugate of q1: {q1.Conjugate()}");
            Console.WriteLine($"Sign number of q1: {q1.Sgn()}");
            Console.WriteLine($"Absolute value of q1: {q1.Abs()}");
            Console.WriteLine($"Inverse of q1: {q1.Inverse()}");
            Console.WriteLine($"Rotating vector {v} by q1: {q1.RotateVector(v)}");
            Console.WriteLine($"Euler angles of q1: {q1.ToEulerAngles()}");
            Console.ReadKey();
        }
    }
}
