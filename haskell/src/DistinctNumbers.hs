module Main where

import Data.ByteString.Builder as B (intDec, toLazyByteString)
import qualified Data.ByteString.Lazy as BS (ByteString, interact)
import Data.ByteString.Lazy.Char8 as C
  ( lines,
    readInt,
    words,
  )
import Data.Maybe (fromJust)
import qualified Data.Set as Set

main :: IO ()
main = BS.interact parse

parse :: BS.ByteString -> BS.ByteString
parse input =
  B.toLazyByteString . B.intDec . solution $ values
  where
    (_first_line : second_line : _) = C.lines input
    numbers = C.words second_line
    values = map (fst . fromJust . C.readInt) numbers

solution :: [Int] -> Int
solution = Set.size . Set.fromList
