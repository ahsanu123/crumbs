import z from "zod";

export const InterpreterSchema = z.object({
  interpreter_id: z.number().int(),
  name: z.string(),
  description: z.string(),
  register_ids: z.array(z.number().int()),
  translator_id: z.number().int(),
});

