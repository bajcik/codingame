#include <stdio.h>
#include <string.h>

typedef struct {
	long tab[501][50];
} stairs_t;

void stairs_init(stairs_t *s)
{
	memset(s, 0xff, sizeof(stairs_t));
}

long stairs_get_count_steps(stairs_t *s, int bricks, int steps)
{
	// cached result
	if (s->tab[bricks][steps] != -1)
		return s->tab[bricks][steps];
	
	fprintf(stderr, "stairs_get_count_steps(%d, %d)...\n", bricks, steps);
	long count = 0;

	if (steps == 1)
		count = 1;
	else
		for (int i=1; i<bricks; i++)
		{
			int bricks2 = bricks - i*steps;
			if (bricks2 <= 0)
				break;
	
			count += stairs_get_count_steps(s, bricks2, steps-1); /// TODO break if returned 0?
		}

	// cache and return
	fprintf(stderr, "stairs_get_count_steps(%d, %d) = %ld\n", bricks, steps, count);
	s->tab[bricks][steps] = count;
	return count;
}


long stairs_get_count(stairs_t *s, int bricks)
{
	long count = 0;
	for (int steps=2;; steps++)
	{
		long step_count = stairs_get_count_steps(s, bricks, steps);
		if (step_count == 0)
			break;
		count += step_count;
	}

	return count;
}

int main()
{
	stairs_t S;
	stairs_init(&S);

	int N;
	scanf("%d", &N);
	printf("%ld\n", stairs_get_count(&S, N));

	return 0;
}

