import { AutoForm, ErrorsField, SubmitField } from "uniforms-mui";
import { ZodBridge } from 'uniforms-bridge-zod';
import { Button, Divider, Drawer, Link, ListItemIcon, ListItemText, MenuList, Stack, Typography } from "@mui/material";
import { useState } from "react";
import { Schema, SchemaTypeBase } from "../schema";
import { TypedAutoField } from "./TypeAutofield";

export default function RegiForm() {
  const schema = new ZodBridge({ schema: Schema });
  const [registerList, setRegisterList] = useState<string[]>([])
  const [isRegisterDrawerOpen, setIsRegisterDrawerOpen] = useState<boolean>(false)

  return (
    <Stack direction={'row'}>
      <Drawer
        open={isRegisterDrawerOpen}
        onClose={() => setIsRegisterDrawerOpen(false)}
      >
        <h1>
          Register List
        </h1>
        <MenuList>

          {registerList.map((reg) => (

            <Link
              href={`#register-${reg.replace(' ', '-')}`}
            >
              <Stack direction='row'>
                <ListItemIcon>
                  o
                </ListItemIcon>

                <ListItemText>
                  <Typography>
                    {reg}
                  </Typography>
                </ListItemText>
              </Stack>

            </Link>

          ))}
        </MenuList>
      </Drawer>


      <AutoForm
        schema={schema}
        onChangeModel={(model: SchemaTypeBase) => {
          if (model.registers === undefined) return;
          const filledRegisterName = model.registers.map((reg) => reg.name).filter((name) => name !== "")
          setRegisterList(filledRegisterName)
        }}
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

      <Button
        onClick={() => setIsRegisterDrawerOpen(true)}
        color="secondary"
        variant="contained"
        aria-label="add"
        sx={{
          position: 'fixed',
          bottom: 20,
          left: 20
        }}
      >
        Registers
      </Button>

    </Stack>
  );
}
