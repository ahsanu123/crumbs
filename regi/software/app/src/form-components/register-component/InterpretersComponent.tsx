import { Button, Divider, Stack, Tab, Tabs, TextField, ToggleButton, Typography } from "@mui/material"
import StarterKit from "@tiptap/starter-kit";
import { MenuButtonBold, MenuButtonItalic, MenuControlsContainer, MenuDivider, MenuSelectHeading, RichTextEditor, RichTextEditorRef } from "mui-tiptap";
import { InterpreterRegisterBitsType, InterpreterType } from "../../schema/interpreter"
import { RegisterType } from "../../schema/register"
import { useEffect, useMemo, useRef, useState } from "react";
import { useRegisterStore } from "../../stores/register-store";

interface InterpreterDescriptionProps {
  description: string
  onSave: (content: string) => void
}

const InterpreterDescription = (props: InterpreterDescriptionProps) => {
  const {
    description,
    onSave
  } = props

  const rteRef = useRef<RichTextEditorRef>(null);

  useEffect(() => {
    rteRef.current?.editor?.commands.setContent(description);
  }, [description]);

  return (
    <Stack>
      <RichTextEditor
        ref={rteRef}
        extensions={[StarterKit]}
        content={description}
        renderControls={() => (
          <MenuControlsContainer>
            <MenuSelectHeading />
            <MenuDivider />
            <MenuButtonBold />
            <MenuButtonItalic />
          </MenuControlsContainer>
        )}
      />

      <Button
        variant="contained"
        onClick={() => {
          const content = rteRef.current?.editor?.getHTML()
          onSave(content ?? "")
        }}
        sx={{
          marginTop: "30px"
        }}
      >
        Save Interpreter Description
      </Button>
    </Stack>
  )
}

interface InterpreterComponentProps {
  register: RegisterType
  interpreters: InterpreterType[]
  onAddInterpreter: () => void
  onRemoveInterpreter: (interpreterId: number) => void
  onUpdateInterpreterRegisterBit: (interpreterId: number, registerId: number, bitId: number) => void
  onChangeInterpreterName: (interpreterId: number, name: string) => void
  onSaveInterpreterDescription: (interpreterId: number, description: string) => void
}

export function InterpreterComponent(props: InterpreterComponentProps) {
  const {
    register,
    interpreters,
    onAddInterpreter,
    onRemoveInterpreter,
    onUpdateInterpreterRegisterBit,
    onChangeInterpreterName,
    onSaveInterpreterDescription
  } = props

  const [selectedInterpreterId, setSelectedInterpreterId] = useState<number | undefined>(undefined)

  const getInterpreterRegisterBits = useRegisterStore(store => store.getInterpreterRegisterBits)

  const selectedInterpreterData = useMemo(() => {
    if (!selectedInterpreterId) return undefined
    return interpreters.find(int => int.interpreter_id === selectedInterpreterId)
  }, [selectedInterpreterId, interpreters])

  const handleOnUpdateInterpreterBits = (interpreterId: number, registerId: number, bitId: number) => {
    onUpdateInterpreterRegisterBit(interpreterId, registerId, bitId)
  }

  const renderInterpreterBits = () => {

    return (
      <>
        {interpreters.map((int, index) => (
          <Stack
            key={index}
          >
            <Stack
              direction={'row'}
            >
              <Typography
                variant="subtitle2"
              >
                {int.name}
              </Typography>
              <Button
                variant="contained"
                onClick={() => onRemoveInterpreter(int.interpreter_id)}
              >
                Remove
              </Button>
            </Stack>

            <Stack
              direction={'row'}
              spacing={2}
              sx={{
                alignItems: 'center',
                margin: "25px 0"
              }}
            >
              {register.bits
                .slice()
                .sort((a, b) => a.bit_ordinal - b.bit_ordinal)
                .reverse()
                .map((bit, index) => (
                  <ToggleButton
                    key={index}
                    fullWidth
                    value={bit.bit_id}
                    onChange={() => handleOnUpdateInterpreterBits(int.interpreter_id, register.register_id, bit.bit_id)}
                    sx={{
                      backgroundColor: getInterpreterRegisterBits(int.interpreter_id, register.register_id)?.bit_ids.includes(bit.bit_id) ? "#95bee8" : ""
                    }}
                  >
                    {`Bit ${bit.bit_ordinal}`}
                  </ToggleButton>
                ))}

            </Stack>
          </Stack>
        ))}
      </>
    )
  }

  return (
    <Stack
      sx={{
        marginTop: "30px"
      }}
    >
      <Stack direction={'row'}>
        <Typography
          variant="h6"
        >
          Interpreters
        </Typography>

        <Button
          variant="contained"
          sx={{ marginLeft: "30px" }}
          onClick={() => onAddInterpreter()}
        >
          Add Interpreter
        </Button>
      </Stack>

      <Stack
        direction={'row'}
      >
        <Stack
          sx={{
            minWidth: "250px"
          }}
        >
          <Tabs
            orientation="vertical"
            variant="scrollable"
            value={selectedInterpreterId}
            onChange={(_, value) => setSelectedInterpreterId(parseInt(value))}
          >
            {interpreters.map((int, index) => (
              <Tab
                key={index}
                label={int.name === "" ? `interpreter ${index}` : int.name}
                value={int.interpreter_id}
              />
            ))}
          </Tabs>

        </Stack>

        <Stack
          sx={{
            width: "100%"
          }}
        >
          {renderInterpreterBits()}

          {!selectedInterpreterId && <Typography>choose or create interpreter to edit</Typography>}

          {selectedInterpreterId && selectedInterpreterData && (
            <Stack
              sx={{
                gap: "40px"
              }}
            >
              <Divider />
              <Typography
                variant="h6"
              >
                {selectedInterpreterData.name}
              </Typography>

              <TextField
                id="interpreter-name"
                label="interpreter name"
                value={selectedInterpreterData.name}
                placeholder="describe interpreter"
                onChange={(ev) => onChangeInterpreterName(selectedInterpreterId, ev.target.value)}
              />

              <Typography
                variant="subtitle2"
              >
                dont forget to save description, without if you change interpreter tab without saving, your will lost your description!!
              </Typography>
              <InterpreterDescription
                description={selectedInterpreterData.description}
                onSave={(content) => onSaveInterpreterDescription(selectedInterpreterId, content)}
              />
            </Stack>
          )}

        </Stack>

      </Stack>
    </Stack>
  )
}
