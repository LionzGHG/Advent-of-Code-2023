import System.IO

solvePartOne :: IO ()
solvePartOne = do
    input <- readFile "day1input.txt" -- get the contents of the input file as string
    -- take the first and last element, if its not null, filter if for number, do that over each line in input
    let calibs = map (\n -> [head n, last n]) $ filter (not . null) $ map (filter (`elem` ['0'..'9'])) $ lines input
    -- take all elements of calibs and sum
    let res = sum $ map read calibs :: Int
    print res

main :: IO ()
main = solvePartOne