#include <stdio.h>

// ----------------------------------------- Program
typedef struct program_t {
    const char *ptr;
} program_t;

void program_init(program_t *prg, const char *str)
{
    prg->ptr = str;
}

#define ISHEX(c) (((c) >= '0' && (c) <= '9') \
               || ((c) >= 'A' && (c) <= 'F'))
#define HEX2INT(c) ((c) < 'A' ?  0+(c)-'0' \
                              : 10+(c)-'A')
// returns 1 when OK
int program_get_opcode(program_t *prg, int *hi, int *lo)
{
    if (!ISHEX(prg->ptr[0]) || !ISHEX(prg->ptr[1]) || !ISHEX(prg->ptr[2]) || !ISHEX(prg->ptr[3]))
        return 0;

    *hi = HEX2INT(prg->ptr[0]) * 0x10 +  HEX2INT(prg->ptr[1]);
    *lo = HEX2INT(prg->ptr[2]) * 0x10 +  HEX2INT(prg->ptr[3]);

    prg->ptr += 4;
    return 1;
}

// ----------------------------------------- CPU
typedef struct cpu_t {
    int reg[3];
    int skipnext;
} cpu_t;

void cpu_init(cpu_t *cpu)
{
    cpu->reg[0] = 0;
    cpu->reg[1] = 0;
    cpu->reg[2] = 0;
    cpu->skipnext = 0;
}

#define REGOK(r) ((r) <= 2)

// returns 0 when program exited
int cpu_exec(cpu_t *cpu, int hi, int lo)
{
    if (cpu->skipnext)
    {
        cpu->skipnext = 0;
        return 1;
    }

     // .-- HH
    // |.-- HL
    // || lo
    // AB CD
    // hi |`-- LL
    //    `-- LH
#    define HH ((hi & 0xf0) >> 4)
#    define HL  (hi & 0x0f)
#    define LH ((lo & 0xf0) >> 4)
#    define LL  (lo & 0x0f)

    switch (hi)
    {
        case 0x00: // EXIT
            return 0;
        break;

        case 0x10: // LD k,nn
        case 0x11:
        case 0x12:
                cpu->reg[HL] = lo;
        break;

        case 0x20: { // ADD x,y
            int x = LH, y = LL;
            if (REGOK(x) && REGOK(y))
            {
                int value =  cpu->reg[x] + cpu->reg[y];
                cpu->reg[x] = value & 0xff;
                cpu->reg[2] = (value & 0x100) >> 8;
            }
        } break;

        case 0x30: { // SUB x,y
            int x = LH, y = LL;
            if (REGOK(x) && REGOK(y))
            {
                int value =  cpu->reg[x] - cpu->reg[y];
                cpu->reg[x] = (value + 0x100) & 0xff;
                cpu->reg[2] = (value & 0x100) >> 8;
            }
        } break;

        case 0x40: { // OR x,y
            int x = LH, y = LL;
            if (REGOK(x) && REGOK(y))
            {
                int value = cpu->reg[x] |  cpu->reg[y];
                cpu->reg[x] = value;
            }
        } break;

        case 0x50: { // AND x,y
            int x = LH, y = LL;
            if (REGOK(x) && REGOK(y))
            {
                int value = cpu->reg[x] &  cpu->reg[y];
                cpu->reg[x] = value;
            }
        } break;

        case 0x60: { // XOR x,y
            int x = LH, y = LL;
            if (REGOK(x) && REGOK(y))
            {
                int value =  cpu->reg[x] ^ cpu->reg[y];
                cpu->reg[x] = value;
            }
        } break;

        case 0x71: // SE k,nn
        case 0x72:
        case 0x73: {
            int k = HL, nn=lo;
            if (cpu->reg[k] == nn)
                cpu->skipnext = 1;
        } break;

        case 0x81: // SNE k,nn
        case 0x82:
        case 0x83: {
            int k = HL, nn=lo;
            if (cpu->reg[k] != nn)
                cpu->skipnext = 1;
        } break;

        case 0x90: { // SE x,y
            int x = LH, y = LL;
            if (REGOK(x) && REGOK(y))
                if (cpu->reg[x] == cpu->reg[y])
                    cpu->skipnext = 1;
        } break;

        case 0xA0: { // SNE x,y
            int x = LH, y = LL;
            if (REGOK(x) && REGOK(y))
                if (cpu->reg[x] != cpu->reg[y])
                    cpu->skipnext = 1;
        } break;
    }
    return 1;
}

void cpu_print_regs(cpu_t *cpu)
{
    printf("%d %d %d\n", cpu->reg[0], cpu->reg[1], cpu->reg[2]);
}

void cpu_debug(cpu_t *cpu)
{
    fprintf(stderr, "R012=0x(%02x,%02x,%02x) / %d,%d,%d%s\n",
        cpu->reg[0], cpu->reg[1], cpu->reg[2],
        cpu->reg[0], cpu->reg[1], cpu->reg[2],
        cpu->skipnext ? " skip" : "");
}

int main()
{
    char program_string[102];
    fgets(program_string, 102, stdin);

    program_t prg;
    program_init(&prg, program_string);

    cpu_t cpu;
    cpu_init(&cpu);

    int hi, lo;
    while (program_get_opcode(&prg, &hi, &lo))
    {
        fprintf(stderr, "opcode=%02x%02x\n", hi, lo);

        if (!cpu_exec(&cpu, hi, lo))
        {
            cpu_print_regs(&cpu);
            break;
        }
        
        cpu_debug(&cpu);
    }
}

