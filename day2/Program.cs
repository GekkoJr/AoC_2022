// See https://aka.ms/new-console-template for more information

Console.WriteLine("Day 2, yay!");

string[] lines  = File.ReadAllLines("input.txt");
int score = 0;

foreach (string line in lines)
{
    char opponent = line[0];
    char you = line[2];

    Console.WriteLine(opponent.ToString());
    Console.WriteLine(you.ToString());

    switch (opponent)
    {
        // stein
        case 'A':
            // stein +1
            if (you == 'X')
            {
                score += 3;
            }
            // papir +2
            if (you == 'Y')
            {
                score += 4;
            }
            // saks +3
            if (you == 'Z')
            {
                score += 8;
            }
            break;
        // papir
        case 'B' :
            // stein +1
            if (you == 'X')
            {
                score += 1;
            }
            // papir +2
            if (you == 'Y')
            {
                score += 5;
            }
            // saks +3
            if (you == 'Z')
            {
                score += 9;
            }
            break;
        // saks
        case'C':
            // stein +1
            if (you == 'X')
            {
                score += 2;
            }
            // papir +2
            if (you == 'Y')
            {
                score += 6;
            }
            // saks +3
            if (you == 'Z')
            {
                score += 7;
            }
            break;
        default:
            Console.WriteLine("noooo");
            break;
    }
}

Console.WriteLine(score.ToString());

