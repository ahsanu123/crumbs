import { Badge, Button, Divider, IconButton, InputLabel, MenuItem, Paper, Select, Stack, Tab, Table, TableBody, TableCell, TableContainer, TableHead, TableRow, Tabs, TextField, ToggleButton, Typography } from "@mui/material"
import { GrRadialSelected } from "react-icons/gr";
import StarterKit from "@tiptap/starter-kit";
import { IoSave } from "react-icons/io5";
import { FaPlus } from "react-icons/fa6";
import { MenuButtonBold, MenuButtonItalic, MenuControlsContainer, MenuDivider, MenuSelectHeading, RichTextEditor, RichTextEditorRef } from "mui-tiptap";
import { InterpreterType, InterpreterTypeEnum, interpreterTypes } from "../../schema/interpreter"
import { FaTrash } from "react-icons/fa";
import { RegisterType } from "../../schema/register"
import { useEffect, useMemo, useRef, useState } from "react";
import { useRegisterStore } from "../../stores/register-store";
import { math } from "../../utility/math"
import { FormulaComponent } from "./FormulaComponent";

interface InterpreterDescriptionProps {
  description: string
  onSave: (content: string) => void
}

function createData(
  name: string,
  calories: number,
  fat: number,
  carbs: number,
  protein: number,
) {
  return { name, calories, fat, carbs, protein };
}

const rows = [
  createData('Frozen yoghurt', 159, 6.0, 24, 4.0),
  createData('Ice cream sandwich', 237, 9.0, 37, 4.3),
  createData('Eclair', 262, 16.0, 24, 6.0),
  createData('Cupcake', 305, 3.7, 67, 4.3),
  createData('Gingerbread', 356, 16.0, 49, 3.9),
];

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
          marginTop: "30px",
          maxWidth: "500px"
        }}
        startIcon={<IoSave />}
      >
        Save Interpreter Description
      </Button>
    </Stack>
  )
}

interface InterpreterComponentProps {
  register: RegisterType
  interpreters: InterpreterType[]
  onUpdateInterpreterType: (interpreterId: number, type: InterpreterTypeEnum) => void
  onAddInterpreter: () => void
  onRemoveInterpreter: (interpreterId: number) => void
  onUpdateInterpreterRegisterBit: (interpreterId: number, registerId: number, bitId: number) => void
  onChangeInterpreterName: (interpreterId: number, name: string) => void
  onSaveInterpreterDescription: (interpreterId: number, description: string) => void
  onUpdateInterpreterFormula: (interpreterId: number, formula: string) => void
}

export function InterpreterComponent(props: InterpreterComponentProps) {
  const {
    register,
    interpreters,
    onAddInterpreter,
    onRemoveInterpreter,
    onUpdateInterpreterRegisterBit,
    onChangeInterpreterName,
    onSaveInterpreterDescription,
    onUpdateInterpreterType,
    onUpdateInterpreterFormula
  } = props

  const [selectedInterpreterId, setSelectedInterpreterId] = useState<number | undefined>(undefined)

  const getInterpreterRegisterBits = useRegisterStore(store => store.getInterpreterRegisterBits)
  const updateMockBitValue = useRegisterStore(store => store.updateMockBitValue)
  const mocks = useRegisterStore(store => store.mocks)

  const selectedInterpreterData = useMemo(() => {
    if (!selectedInterpreterId) return undefined
    return interpreters.find(int => int.interpreter_id === selectedInterpreterId)
  }, [selectedInterpreterId, interpreters])

  const handleOnUpdateInterpreterBits = (interpreterId: number, registerId: number, bitId: number) => {
    onUpdateInterpreterRegisterBit(interpreterId, registerId, bitId)
  }

  const mockData = useMemo(() => {
    if (!selectedInterpreterId) return undefined
    return mocks.find(mock => mock.register_id === register.register_id && mock.interpreter_id === selectedInterpreterId)
  }, [register, selectedInterpreterId])

  useEffect(() => {
    if (interpreters.length > 0 && selectedInterpreterId === undefined)
      setSelectedInterpreterId(1)

  }, [interpreters])

  const renderBitCondition = () => (
    <>
      <TableContainer component={Paper}>
        <Table>
          <TableHead>
            <TableRow>
              <TableCell>Dessert (100g serving)</TableCell>
              <TableCell align="right">Calories</TableCell>
              <TableCell align="right">Fat&nbsp;(g)</TableCell>
              <TableCell align="right">Carbs&nbsp;(g)</TableCell>
              <TableCell align="right">Protein&nbsp;(g)</TableCell>
            </TableRow>
          </TableHead>
          <TableBody>
            {rows.map((row) => (
              <TableRow
                key={row.name}
                sx={{ '&:last-child td, &:last-child th': { border: 0 } }}
              >
                <TableCell component="th" scope="row">
                  {row.name}
                </TableCell>
                <TableCell align="right">{row.calories}</TableCell>
                <TableCell align="right">{row.fat}</TableCell>
                <TableCell align="right">{row.carbs}</TableCell>
                <TableCell align="right">{row.protein}</TableCell>
              </TableRow>
            ))}
          </TableBody>
        </Table>
      </TableContainer>

    </>
  )

  const renderInterpreterDescriptionAndFormula = () => (
    <Stack
      direction={'row'}
      sx={{
        gap: "50px"
      }}
    >
      <Stack
        sx={{
          maxWidth: "40%"
        }}
      >
        <Typography
          variant="subtitle2"
        >
          dont forget to save description, without if you change interpreter tab without saving, your will lost your description!!
        </Typography>
        <InterpreterDescription
          description={selectedInterpreterData?.description ?? ""}
          onSave={(content) => selectedInterpreterId && onSaveInterpreterDescription(selectedInterpreterId, content)}
        />
      </Stack>

      <Stack>
        <InputLabel id="interpreter-types">Interpreter Type</InputLabel>
        <Select
          labelId="interpreter-types"
          id="interpreter-types"
          label="Interpreter Type"
          value={selectedInterpreterData?.type}
          onChange={(ev) => selectedInterpreterId && onUpdateInterpreterType(selectedInterpreterId, ev.target.value as InterpreterTypeEnum)}
        >
          {interpreterTypes.map((intType) => (
            <MenuItem value={intType}>{intType}</MenuItem>
          ))}
        </Select>

        {selectedInterpreterData?.type === 'Formula' && mockData && (
          <FormulaComponent
            register={register}
            interpreter={selectedInterpreterData}
            mock={mockData}
            onMockDataChange={(mockId, bitId, data) => updateMockBitValue(mockId, bitId, data)}
            onUpdateFormula={(interpreterId, formula) => onUpdateInterpreterFormula(interpreterId, formula)}
          />
        )}

        {selectedInterpreterData?.type === 'BitCondition' && renderBitCondition()}

      </Stack>

    </Stack>
  )

  const renderInterpreterBits = () => {

    return (
      <>
        {interpreters.map((int, index) => (
          <Stack
            key={index}
          >
            <Stack
              direction={'row'}
              sx={{
                alignItems: 'center',
                justifyContent: 'space-between'
              }}
            >
              <Stack direction={'row'}>
                {selectedInterpreterId === int.interpreter_id && (
                  <IconButton disabled>
                    <GrRadialSelected color="#78ba66" />
                  </IconButton>
                )}

                <Typography
                  variant="h5"
                >
                  {int.name === "" ? "Please Enter Interpreter Name" : int.name}
                </Typography>
              </Stack>

              <Button
                variant="contained"
                color="secondary"
                onClick={() => onRemoveInterpreter(int.interpreter_id)}
                startIcon={<FaTrash />}
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
        margin: "30px 0"
      }}
    >
      <Stack
        direction={'row'}
        sx={{
          margin: "30px 10px",
          justifyContent: 'start'
        }}
      >
        <Button
          variant="contained"
          sx={{ marginRight: "30px" }}
          onClick={() => onAddInterpreter()}
          startIcon={<FaPlus />}
        >
          Add Interpreter
        </Button>

        <Typography
          variant="h5"
        >
          Interpreters
        </Typography>
      </Stack>

      <Stack
        direction={'row'}
        sx={{
          gap: "40px"
        }}
      >
        <Stack
          sx={{
            minWidth: "250px",
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
            width: "100%",
          }}
        >
          {!selectedInterpreterId && <Typography>choose or create interpreter to edit</Typography>}

          {selectedInterpreterId && selectedInterpreterData && (
            <Stack
              sx={{
                gap: "40px"
              }}
            >
              <Divider />

              <TextField
                id="interpreter-name"
                label="interpreter name"
                value={selectedInterpreterData.name}
                placeholder="describe interpreter"
                onChange={(ev) => onChangeInterpreterName(selectedInterpreterId, ev.target.value)}
                sx={{
                  maxWidth: "700px"
                }}
              />

              {renderInterpreterDescriptionAndFormula()}

              {renderInterpreterBits()}

            </Stack>
          )}

        </Stack>

      </Stack>
    </Stack>
  )
}
