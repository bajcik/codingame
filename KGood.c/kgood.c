#include <stdio.h>
#include <string.h>

typedef struct 
{
    int chn[27]; // [0] for 'a', [1] for 'b' ...
    int K;       // number of different chars
    int len;
} KState;

void KState_init(KState *ks)
{
    ks->len = 0;
    ks->K = 0;
    for (int i=0; i<27; i++)
        ks->chn[i] = 0;
}

void KState_push(KState *ks, char ch)
{
    int idx = ch - 'a';
    
    // new letter?
    if (!ks->chn[idx])
        ks->K++;
    ks->chn[idx]++;
    ks->len++;
}

void KState_pop(KState *ks, char ch)
{
    int idx = ch - 'a';
    
    ks->chn[idx]--;
    
    // removed letter?
    if (!ks->chn[idx])
        ks->K--;
    ks->len--;
}

void KState_print(KState *ks, char *str)
{
    fprintf(stderr, "K=%d len=%d", ks->K, ks->len);
    for (int i=0; i<27; i++)
        if (ks->chn[i])
            fprintf(stderr, ",%c=%d", i+'a', ks->chn[i]);
    
    fprintf(stderr, "\nstring=(");
    for (int i=0; i<ks->len; i++)
        fputc(str[i], stderr);
    fprintf(stderr, ")\n");
}

int main()
{
    char S[5001]; fgets(S, 5001, stdin);
    int K; scanf("%d", &K);
    int lenS = strlen(S);
    
    KState ks;
    KState_init(&ks);

    int start = 0; // idx of last added char
    int end = 0;   // idx of next char to add
    int maxlen = 0;
    
    while (end < lenS)
    {
        if (ks.K <= K) // still KGood
        {
            if (ks.len > maxlen)
            {
                maxlen = ks.len;
                //KState_print(&ks, S+start);
            }
            KState_push(&ks, S[end++]);
        }
        else // not KGood (but K+1Good)
        {
            KState_pop(&ks, S[start++]);
        }
    }
    
    // Write an action using printf(). DON'T FORGET THE TRAILING \n
    // To debug: fprintf(stderr, "Debug messages...\n");

    printf("%d\n", maxlen);

    return 0;
}
