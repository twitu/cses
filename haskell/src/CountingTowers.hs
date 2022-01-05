{-# LANGUAGE DeriveFunctor #-}
{-# LANGUAGE NumericUnderscores #-}

module CountingTowers where

import Control.Arrow hiding (left, right)
import Data.ByteString.Builder as B (charUtf8, intDec, toLazyByteString)
import qualified Data.ByteString.Lazy as BS (ByteString, interact)
import Data.ByteString.Lazy.Char8 as C
  ( lines,
    readInt,
  )
import Data.Maybe (fromJust)
import Prelude hiding (lookup, unlines)

main :: IO ()
main = BS.interact parse

parse :: BS.ByteString -> BS.ByteString
parse input =
  B.toLazyByteString
    . foldMap (B.charUtf8 '\n' `mappend`)
    . map (B.intDec . solution)
    $ values
  where
    values =
      map
        ( (subtract 1)
            . fst
            . fromJust
            . C.readInt
        )
        . tail
        . C.lines
        $ input

newtype Term f = In {out :: f (Term f)}

data Attr f a = Attr
  { attribute :: a,
    hole :: f (Attr f a)
  }

type CVAlgebra f a = f (Attr f a) -> a

histo :: Functor f => CVAlgebra f a -> Term f -> a
histo h = out >>> fmap worker >>> h
  where
    worker t = Attr (histo h t) (fmap worker (out t))

data Nat a
  = Zero
  | Next a
  deriving (Functor)

-- Convert from a natural number to its foldable equivalent, and vice versa.
expand :: Int -> Term Nat
expand 0 = In Zero
expand n = In (Next (expand (n - 1)))

compress :: Nat (Attr Nat a) -> Int
compress Zero = 0
compress (Next (Attr _ x)) = 1 + compress x

type TwoBlocksCount = Int

type OneBlockCount = Int

type StoryInfo = (TwoBlocksCount, OneBlockCount)

modAns :: Int -> Int
modAns = (`rem` 1_000_000_007)

solution :: Int -> Int
solution story = modAns . uncurry (+) . histo go $ expand story
  where
    go :: Nat (Attr Nat StoryInfo) -> StoryInfo
    go Zero = (1, 1)
    go (Next attr) =
      let (twoBlocks, oneBlock) = lookup attr 1
       in ( modAns (twoBlocks * 4 + oneBlock),
            modAns (twoBlocks + oneBlock * 2)
          )
    lookup :: Attr Nat a -> Int -> a
    lookup cache 1 = attribute cache
    lookup cache n = lookup inner (n - 1) where (Next inner) = hole cache
