import { AutoField, AutoFieldProps, AutoForm, ErrorsField, SubmitField } from "uniforms-mui";
import { RegisterSchema, SchemaSchema } from "../schema";
import { ZodBridge } from 'uniforms-bridge-zod';
import { z } from "zod";
import { Divider, Stack, Typography } from "@mui/material";
import { useState } from "react";
import { HTMLFieldProps } from "uniforms";

type SchemaType = z.infer<typeof SchemaSchema>;
type SchemaKeys = keyof SchemaType;
type RegisterTypeT = z.infer<typeof RegisterSchema>;

type RegisterType =
  HTMLFieldProps<RegisterTypeT, HTMLDivElement>;

const TypedAutoField = (props: AutoFieldProps & { name: SchemaKeys }) => (
  <AutoField {...props} />
)

// onChangeModel={(model, details) => console.log("model changed", model, details)}
export default function RegiForm() {
  const schema = new ZodBridge({ schema: SchemaSchema });
  const [registerList, setRegisterList] = useState<string[]>([])

  return (
    <Stack direction={'row'}>
      <Stack>
        <h1>
          Register List
        </h1>

        {registerList.map((reg) => (
          <Typography>
            {reg}
          </Typography>
        ))}

      </Stack>

      <AutoForm
        schema={schema}
        onChangeModel={(model) => console.log("modelchanged: ", model)}
        onSubmit={(model: any) => window.alert(JSON.stringify(model))}
      >

        <TypedAutoField name="name" />
        <TypedAutoField name="description" />
        <TypedAutoField name="version" />

        <TypedAutoField name="registers" />

        <ErrorsField />

        <Divider />
        <SubmitField />
      </AutoForm>

    </Stack>
  );
}
