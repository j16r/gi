#ifndef _ENGINE_INSTRUCTIONS_H_
#define _ENGINE_INSTRUCTIONS_H_

int engine_instruction_return(Engine_t *, char *);
void engine_instruction_noop(Engine_t *);
void engine_instruction_not_supported(Engine_t *);
void engine_instruction_define_function(Engine_t *, char *);

#endif
