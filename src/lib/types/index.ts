import {z} from "zod";

const LetterStateSchema = z.enum(["unknown", "correct", "misplaced", "absent"]);
const CharacterSchema = z
  .string()
  .min(1, {message: "Letter must be at least 1 character"})
  .max(1, {message: "Letter must be at most 1 character"})
  .refine((val) => /^[a-zA-Z]$/.test(val), {
    message: "Input must be a single letter"
  })
  .transform((val) => val.toLowerCase());

const LetterSchema = z.object({
  character: CharacterSchema,
  state: LetterStateSchema,
});

const WordSchema = z
  .array(LetterSchema)
  .refine((letters) => letters.length === 5, {
    message: "Word must be 5 letters",
  });

const FinalLetterStateSchema = z.enum(["correct", "misplaced", "absent"]);
const FinalLetterSchema = z.object({
  character: CharacterSchema,
  state: FinalLetterStateSchema,
});

const FinalWordSchema = z
  .array(FinalLetterSchema)
  .refine((letters) => letters.length === 5, {
    message: "Word must be 5 letters",
  });

export const GuessesSchema = z.array(WordSchema);
export type TGuesses = z.infer<typeof GuessesSchema>;

export type TWord = z.infer<typeof WordSchema>;
