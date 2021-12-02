#include <stdio.h>
#include <stdlib.h>
#include <string.h>

typedef struct
{
	int pos;
	int height;
} position_T;

typedef struct
{
	int pos;
	int height;
	int aim;
} movement_T;

void solution(const char *file_path);
int part_1(FILE *file);
int part_2(FILE *file);
void parse_movement(position_T *pos, char *direction, int amount);
void parse_movement_for_real(movement_T *mov, char *direction, int amount);

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

	printf("Part 1: The new position is: %i\n", part_1(file));
	rewind(file);
	printf("Part 2: The new position is: %i\n", part_2(file));
}

int part_1(FILE *file)
{
	int read;
	char *line = NULL;
	size_t len = 0;
	position_T position = {
		.pos = 0,
		.height = 0,
	};
	char space = ' ';
	char nl = '\n';

	while ((read = getline(&line, &len, file)) != -1)
	{
		char *pPos = strchr(line, space);
		if (pPos == NULL)
		{
			break;
		}
		int spos = pPos - line;
		char *direction = (char *)malloc((spos + 1) * sizeof(char));
		memcpy(direction, line, spos);
		pPos = strchr(line, nl);
		if (pPos == NULL)
		{
			break;
		}
		int nlpos = pPos - line;
		char *mov_amount = (char *)malloc((nlpos - spos + 1) * sizeof(char));
		memcpy(mov_amount, line + spos, nlpos);
		int move_amount = atoi(mov_amount);
		parse_movement(&position, direction, move_amount);
	}

	return position.pos * position.height;
}

void parse_movement(position_T *pos, char *direction, int amount)
{
	if (!strcmp(direction, "forward"))
	{
		pos->pos += amount;
	}
	else if (!strcmp(direction, "up"))
	{
		pos->height -= amount;
	}
	else if (!strcmp(direction, "down"))
	{
		pos->height += amount;
	}
}

int part_2(FILE *file)
{
	int read;
	char *line = NULL;
	size_t len = 0;
	movement_T mov = {
		.pos = 0,
		.height = 0,
		.aim = 0,
	};
	char space = ' ';
	char nl = '\n';

	while ((read = getline(&line, &len, file)) != -1)
	{
		char *pPos = strchr(line, space);
		if (pPos == NULL)
		{
			break;
		}
		int spos = pPos - line;
		char *direction = (char *)malloc((spos + 1) * sizeof(char));
		memcpy(direction, line, spos);
		pPos = strchr(line, nl);
		if (pPos == NULL)
		{
			break;
		}
		int nlpos = pPos - line;
		char *mov_amount = (char *)malloc((nlpos - spos + 1) * sizeof(char));
		memcpy(mov_amount, line + spos, nlpos);
		int move_amount = atoi(mov_amount);
		parse_movement_for_real(&mov, direction, move_amount);
	}

	return mov.pos * mov.height;
}

void parse_movement_for_real(movement_T *mov, char *direction, int amount)
{
	if (!strcmp(direction, "forward"))
	{
		mov->pos += amount;
		mov->height += amount * mov->aim;
	}
	else if (!strcmp(direction, "up"))
	{
		mov->aim -= amount;
	}
	else if (!strcmp(direction, "down"))
	{
		mov->aim += amount;
	}
}