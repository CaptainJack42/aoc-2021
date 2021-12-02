#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>

typedef struct
{
	int val1;
	int val2;
	int val3;
	int sum;
} data_frame_T;

void solve_file(const char *file_path);
int part_1(FILE *file);
int part_2(FILE *file);
int compare(data_frame_T *prev, data_frame_T *curr, int counter);

int main(int argc, char *argv[])
{
	for (int i = 1; i < argc; i++)
	{
		solve_file(argv[i]);
	}
}

void solve_file(const char *file_path)
{

	FILE *file = fopen(file_path, "r");

	if (file == NULL)
	{
		printf("error opening file '%s' !\n", file_path);
		return;
	}

	printf("Part 1: There are %i measurements larger than the previous one. \n", part_1(file));
	rewind(file);
	printf("Part 2: There are %i data frames larger than the previous one!", part_2(file));

	fclose(file);
}

int part_1(FILE *file)
{
	int read;
	char *line = NULL;
	size_t len = 0;
	int val;
	int prev_val;
	int count_greater = 0;

	read = getline(&line, &len, file);
	prev_val = atoi(line);

	while ((read = getline(&line, &len, file)) != -1)
	{
		val = atoi(line);
		if (val > prev_val)
		{
			count_greater++;
		}
		prev_val = val;
	}
	return count_greater;
}

int part_2(FILE *file)
{
	int count_greater = 0;
	data_frame_T f1;
	data_frame_T f2;
	data_frame_T f3;
	data_frame_T f4;
	int cycle_count = 0;
	int read;
	char *line = NULL;
	size_t len = 0;

	while ((read = getline(&line, &len, file)) != -1)
	{
		int val = atoi(line);

		switch (cycle_count % 4)
		{
		case 0:
			f1.val1 = val;
			f3.val3 = val;
			f4.val2 = val;
			count_greater += compare(&f2, &f3, cycle_count);
			cycle_count++;
			break;
		case 1:
			f1.val2 = val;
			f2.val1 = val;
			f4.val3 = val;
			count_greater += compare(&f3, &f4, cycle_count);
			cycle_count++;
			break;

		case 2:
			f1.val3 = val;
			f2.val2 = val;
			f3.val1 = val;
			count_greater += compare(&f4, &f1, cycle_count);
			cycle_count++;
			break;

		case 3:
			f2.val3 = val;
			f3.val2 = val;
			f4.val1 = val;
			count_greater += compare(&f1, &f2, cycle_count);
			cycle_count++;
			break;
		default:
			break;
		}
	}

	return count_greater;
}

int compare(data_frame_T *prev, data_frame_T *curr, int counter)
{
	curr->sum = curr->val1 + curr->val2 + curr->val3;

	if (counter < 3)
	{
		return 0;
	}
	if (curr->sum > prev->sum)
	{
		return 1;
	}
	else
	{
		return 0;
	}
}