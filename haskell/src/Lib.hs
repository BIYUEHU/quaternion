module Lib (Quaternion, Vector3, newQuaternion, newVector3, add, sub, mul, conjugate, divide, scalarProduct, outerProduct, evenProduct, crossProduct, rotateVector, mulVector, absQuaternion, sgnQuaternion, inverseQuaternion, argQuaternion, toEulerAngles, toVector3, showQuaternion) where

data Quaternion = Quaternion
  { w :: Double,
    i :: Double,
    j :: Double,
    k :: Double
  }
  deriving (Show)

data Vector3 = Vector3 Double Double Double deriving (Show)

newQuaternion :: Double -> Double -> Double -> Double -> Quaternion
newQuaternion = Quaternion

newVector3 :: Double -> Double -> Double -> Vector3
newVector3 = Vector3

add :: Quaternion -> Quaternion -> Quaternion
add (Quaternion w1 i1 j1 k1) (Quaternion w2 i2 j2 k2) =
  Quaternion (w1 + w2) (i1 + i2) (j1 + j2) (k1 + k2)

sub :: Quaternion -> Quaternion -> Quaternion
sub (Quaternion w1 i1 j1 k1) (Quaternion w2 i2 j2 k2) =
  Quaternion (w1 - w2) (i1 - i2) (j1 - j2) (k1 - k2)

mul :: Quaternion -> Quaternion -> Quaternion
mul (Quaternion w1 i1 j1 k1) (Quaternion w2 i2 j2 k2) =
  Quaternion
    (w1 * w2 - i1 * i2 - j1 * j2 - k1 * k2)
    (w1 * i2 + w2 * i1 + j1 * k2 - j2 * k1)
    (w1 * j2 + w2 * j1 + k1 * i2 - k2 * i1)
    (w1 * k2 + w2 * k1 + i1 * j2 - i2 * j1)

conjugate :: Quaternion -> Quaternion
conjugate (Quaternion w1 i1 j1 k1) =
  Quaternion w1 (- i1) (- j1) (- k1)

divide :: Quaternion -> Quaternion -> Quaternion
divide q1 q2 = mul q1 (conjugate q2)

scalarProduct :: Quaternion -> Quaternion -> Double
scalarProduct (Quaternion w1 i1 j1 k1) (Quaternion w2 i2 j2 k2) =
  w1 * w2 + i1 * i2 + j1 * j2 + k1 * k2

outerProduct :: Quaternion -> Quaternion -> Quaternion
outerProduct (Quaternion w1 i1 j1 k1) (Quaternion w2 i2 j2 k2) =
  Quaternion
    (w1 * w2 - i1 * i2 - j1 * j2 - k1 * k2)
    (w1 * i2 + w2 * i1 + j1 * k2 - j2 * k1)
    (w1 * j2 - w2 * j1 + k1 * i2 + k2 * i1)
    (w1 * k2 + w2 * k1 + i1 * j2 + i2 * j1)

evenProduct :: Quaternion -> Quaternion -> Quaternion
evenProduct (Quaternion w1 i1 j1 k1) (Quaternion w2 i2 j2 k2) =
  Quaternion
    (w1 * w2 - i1 * i2 - j1 * j2 - k1 * k2)
    (w1 * i2 + w2 * i1 - j1 * k2 + j2 * k1)
    (w1 * j2 + w2 * j1 + k1 * i2 - k2 * i1)
    (w1 * k2 - w2 * k1 + i1 * j2 + i2 * j1)

crossProduct :: Quaternion -> Quaternion -> Quaternion
crossProduct (Quaternion w1 i1 j1 k1) (Quaternion w2 i2 j2 k2) =
  Quaternion
    (w1 * w2 - i1 * i2 - j1 * j2 - k1 * k2)
    (w1 * i2 + w2 * i1 + j1 * k2 - j2 * k1)
    (w1 * j2 - w2 * j1 + k1 * i2 + k2 * i1)
    (w1 * k2 + w2 * k1 - i1 * j2 + i2 * j1)

rotateVector :: Quaternion -> Vector3 -> Vector3
rotateVector q1 (Vector3 x y z) =
  let (Quaternion _ resultI resultJ resultK) = mul (Quaternion 0 x y z) (conjugate q1)
   in Vector3 resultI resultJ resultK

mulVector :: Quaternion -> Vector3 -> Vector3
mulVector q1 (Vector3 x y z) =
  let (Quaternion _ resultI resultJ resultK) = mul (Quaternion 0 x y z) (conjugate q1)
   in Vector3 resultI resultJ resultK

absQuaternion :: Quaternion -> Double
absQuaternion (Quaternion w1 i1 j1 k1) = sqrt (w1 ** 2 + i1 ** 2 + j1 ** 2 + k1 ** 2)

sgnQuaternion :: Quaternion -> Quaternion
sgnQuaternion q1 =
  let absValue = absQuaternion q1
      (Quaternion w1 i1 j1 k1) = q1
   in Quaternion (w1 / absValue) (i1 / absValue) (j1 / absValue) (k1 / absValue)

inverseQuaternion :: Quaternion -> Quaternion
inverseQuaternion q1 =
  let abs2Value = absQuaternion q1 ** 2
      (Quaternion w1 i1 j1 k1) = q1
   in Quaternion (w1 / abs2Value) (- i1 / abs2Value) (- j1 / abs2Value) (- k1 / abs2Value)

argQuaternion :: Quaternion -> Double
argQuaternion q1 =
  let (Quaternion w1 _ _ _) = q1
   in acos w1 / absQuaternion q1

toEulerAngles :: Quaternion -> (Double, Double, Double)
toEulerAngles (Quaternion w1 i1 j1 k1) =
  let num1 = 2 * (w1 * i1 + j1 * k1)
      num2 = 1 - 2 * (i1 ** 2 + j1 ** 2)
      num3 = 2 * (w1 * j1 - k1 * i1)
   in (atan2 num1 num2, asin num3, atan2 num1 num2)

toVector3 :: Quaternion -> Vector3
toVector3 (Quaternion _ i1 j1 k1) = Vector3 i1 j1 k1

showQuaternion :: Quaternion -> String
showQuaternion (Quaternion w1 i1 j1 k1) =
  let getSign = \num -> if num >= 0 then "+" else "-"
   in show w1 ++ getSign i1 ++ show (abs i1) ++ "i" ++ getSign j1 ++ show (abs j1) ++ "j" ++ getSign k1 ++ show (abs k1) ++ "k"

instance Num Quaternion where
  (+) = add
  (-) = sub
  (*) = mul
  negate (Quaternion w1 i1 j1 k1) = Quaternion (- w1) (- i1) (- j1) (- k1)
  abs q1 = Quaternion (absQuaternion q1) 0 0 0
  signum = sgnQuaternion
  fromInteger n = Quaternion (fromInteger n) 0 0 0
