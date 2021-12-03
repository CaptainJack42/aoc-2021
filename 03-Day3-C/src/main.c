#include <stdbool.h>
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

void solution(const char *file_path);
int part_1(FILE *file);
int part_2(FILE *file);

int main(int argc, char *argv[])
{
	for (int i = 1; i < argc; i++)
	{
		solution(argv[i]);
	}
}

void solution(const char *file_path)
{
	FILE *file = fopen(file_path, "r");

	if (file == NULL)
	{
		printf("error opening %s!", file_path);
		return;
	}

	printf("Part 1: The power consumption is: %i\n", part_1(file));
	rewind(file);
	printf("Part 2: The life support rating is: %i\n", part_2(file));
}

int part_1(FILE *file)
{
	int read;
	char *line = NULL;
	size_t len = 0;
	int line_len = getline(&line, &len, file) - 1;
	rewind(file);
	int bitcounter[line_len];
	memset(bitcounter, 0, sizeof(int) * line_len);
	int linecounter = 0;

	while ((read = getline(&line, &len, file)) != -1)
	{
		for (int i = 0; i < read; i++)
		{
			char *ch = (char *)malloc(1 * sizeof(char));
			memcpy(ch, line + i, 1);
			if (!strcmp(ch, "1"))
			{
				bitcounter[i]++;
			}
		}
		linecounter++;
	}

	int gamma_rate = 0;
	int epsilon_rate = 0;

	for (int i = 0; i < line_len; i++)
	{
		if (bitcounter[i] > linecounter / 2)
		{
			gamma_rate = gamma_rate | (1 << (line_len - i - 1));
		}
		else
		{
			epsilon_rate = epsilon_rate | (1 << (line_len - i - 1));
		}
	}

	return gamma_rate * epsilon_rate;
}

int part_2(FILE *file)
{
	int read;
	char *line = NULL;
	size_t len = 0;
	int line_len = getline(&line, &len, file) - 1;
	rewind(file);
	int bitcounter[line_len];
	memset(bitcounter, 0, sizeof(int) * line_len);
	int linecounter = 0;

	while ((read = getline(&line, &len, file)) != -1)
	{
		linecounter++;
	}

	rewind(file);
	unsigned int values[linecounter];
	linecounter = 0;
	char *endp = NULL;

	while ((read = getline(&line, &len, file)) != -1)
	{
		values[linecounter] = strtoul(line, &endp, 2);
		linecounter++;
	}

	bool viable_ox[linecounter];
	memset(viable_ox, true, sizeof(bool) * linecounter);
	bool viable_co2[linecounter];
	memset(viable_co2, true, sizeof(bool) * linecounter);

	int ox_rating = 0;
	int co2_rating = 0;
	int count_viable_co2 = linecounter;
	int count_viable_ox = linecounter;

	for (int i = line_len - 1; i >= 0; i--)
	{
		int count_ones_ox = 0;
		int count_zeroes_ox = 0;
		int count_ones_co2 = 0;
		int count_zeroes_co2 = 0;

		for (int j = 0; j < linecounter; j++)
		{
			if (viable_ox[j])
			{
				if (values[j] & (1 << i))
				{
					count_ones_ox++;
				} else {
					count_zeroes_ox++;
				}
				// count_viable_ox++;
			}
			if (viable_co2[j])
			{
				if (!(values[j] & (1 << i)))
				{
					count_zeroes_co2++;
				} else {
					count_ones_co2++;
				}
				// count_viable_co2++;
			}
		}

		int sign_ox = 0;
		int sign_co2 = 1;
		if (count_ones_ox >= count_zeroes_ox)
		{
			sign_ox = 1;
		}
		if (count_ones_co2 >= count_zeroes_co2)
		{
			sign_co2 = 0;
		}

		for (int j = 0; j < linecounter; j++)
		{
			if (((sign_ox == 0 && (values[j] & (1 << i))) ||
				 (sign_ox == 1 && !(values[j] & (1 << i)))) &&
				viable_ox[j])
			{
				viable_ox[j] = false;
				count_viable_ox--;
			}
			if (((sign_co2 == 0 && (values[j] & (1 << i))) ||
				 (sign_co2 == 1 && !(values[j] & (1 << i)))) &&
				viable_co2[j])
			{
				viable_co2[j] = false;
				count_viable_co2--;
			}
		}

		if (count_viable_ox == 1)
		{
			for (int j = 0; j < linecounter; j++)
			{
				if (viable_ox[j])
				{
					ox_rating = values[j];
					break;
				}
			}
		}

		if (count_viable_co2 == 1)
		{
			for (int j = 0; j < linecounter; j++)
			{
				if (viable_co2[j])
				{
					co2_rating = values[j];
					break;
				}
			}
		}

		if ((ox_rating > 0) && (co2_rating > 0))
		{
			break;
		}
	}

	return ox_rating * co2_rating;
}