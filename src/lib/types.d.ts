import {z} from "zod";

const LetterStateSchema = z.enum(["unknown", "correct", "misplaced", "absent"]);

const LetterSchema = z.object({
  character: z
    .string()
    .min(1, {message: "Letter must be at least 1 character"})
    .max(1, {message: "Letter must be at most 1 character"})
    .transform((val) => val.toLowerCase())
    .regex(/^[a-z]$/, {message: "Letter must be a single letter"}),

  state: LetterStateSchema,
});

const WordSchema = z
  .array(LetterSchema)
  .refine((letters) => letters.length === 5, {
    message: "Word must be 5 letters",
  });


export type TLetter = z.infer<typeof LetterSchema>;
export type TWord = z.infer<typeof WordSchema>;
