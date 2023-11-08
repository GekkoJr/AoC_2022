// See https://aka.ms/new-console-template for more information

Console.WriteLine("Calculation shit");

string[] lines = File.ReadAllLines("/home/gekko/programmering/AoC/2022/1/input.txt");

var list = new List<int>();
int currentElf = 0;

foreach (string line in lines)
{
    if (line == "")
    {
        list.Add(currentElf);
        currentElf = 0;
    }
    else
    {
        int currentNum = int.Parse(line);
        currentElf += currentNum;
    }
}
list.Sort();
list.Reverse();

int output = 0;

for (int i = 0; i <= 2; i++)
{
    output += list[i];
}

Console.WriteLine("The result is: " + output.ToString());

