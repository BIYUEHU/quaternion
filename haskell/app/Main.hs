module Main (main) where

import Lib (absQuaternion, conjugate, divide, inverseQuaternion, newQuaternion, newVector3, rotateVector, sgnQuaternion, showQuaternion, toEulerAngles, toVector3)

main :: IO ()
main =
  let q1 = newQuaternion 1 1 1 1
      q2 = newQuaternion 0 1 0 1
   in do
        putStrLn $ "q1: " ++ showQuaternion q1
        putStrLn $ "q1 to vector: " ++ show (toVector3 q1)
        putStrLn $ "q1 + q2 = " ++ show (q1 + q2)
        putStrLn $ "q1 * q2 = " ++ show (q1 * q2)
        putStrLn $ "q1 - q2 = " ++ show (q1 - q2)
        putStrLn $ "q1 / q2 = " ++ show (divide q1 q2)
        putStrLn $ "Conjugate of q1: " ++ show (conjugate q1)
        putStrLn $ "Sign number of q1: " ++ show (sgnQuaternion q1)
        putStrLn $ "Absolute value of q1: " ++ show (absQuaternion q1)
        putStrLn $ "Inverse of q1: " ++ show (inverseQuaternion q1)
        putStrLn $ "Rotating vector (1, 0, 0) by q1: " ++ show (rotateVector q1 (newVector3 1 0 0))
        putStrLn $ "Euler angles of q1: " ++ show (toEulerAngles q1)
