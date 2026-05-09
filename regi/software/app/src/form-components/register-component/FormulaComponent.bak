import { Button, Divider, Stack, Table, TableBody, TableCell, TableContainer, TableHead, TableRow, TextField, Typography } from "@mui/material";
import { InterpreterType } from "../../schema/interpreter";
import { MockType } from "../../schema/mock";
import { RegisterType } from "../../schema/register";
import { useMemo, useState } from "react";
import { evaluateEquation } from "../../utility/math"

interface FormulaComponentProps {
  register: RegisterType
  interpreter: InterpreterType
  mock: MockType
  onMockDataChange: (mockId: number, bitId: number, data: string) => void
  onUpdateFormula: (interpreterId: number, formula: string) => void
}

export function FormulaComponent(props: FormulaComponentProps) {
  const {
    register,
    interpreter,
    mock,
    onMockDataChange,
    onUpdateFormula,
  } = props

  const [errorMessage, setErrorMessage] = useState<string | undefined>(undefined)
  const [equationResult, setEquationResult] = useState<string | undefined>(undefined)

  const mockScopeDataKeys = mock.datas.map(mockData => `bit${mockData.bit_id}`)

  const handleOnEvaluateFormula = () => {
    if (!interpreter.formula) return;

    const mockScopeData = Object.fromEntries(
      mock.datas.map(mockData => [
        `bit${mockData.bit_id}`,
        mockData.value
      ])
    )
    console.log("scope data", mockScopeData)
    const evalResult = evaluateEquation(interpreter.formula, mockScopeData)

    setErrorMessage(evalResult.errorMessage)
    setEquationResult(JSON.stringify(evalResult.result))
  }

  const bitInterpreterRegister = useMemo(() => {
    return interpreter.registers.filter(reg => reg.register_id === register.register_id).map(reg => reg.bit_ids).flat()
  }, [register, interpreter])

  return (
    <>
      <TableContainer>
        <Table>
          <TableHead>
            <TableRow>
              <TableCell>
                <b>Mocks</b>
              </TableCell>
              {Array.from({ length: bitInterpreterRegister.length }).map((_, index) => index).reverse().map((num) => (
                <TableCell
                  align="center"
                  key={num}
                >
                  <b>
                    {num}
                  </b>
                </TableCell>
              ))}
            </TableRow>
          </TableHead>
          <TableBody>

            <TableRow>
              <TableCell>
                <b>Mock Value</b>
              </TableCell>
              {bitInterpreterRegister
                .slice()
                .sort((a, b) => a - b)
                .reverse()
                .map((bitId, index) => (
                  <TableCell
                    align="center"
                    key={bitId}
                  >
                    <TextField
                      id={`formula-mock-${index}`}
                      label="mock value"
                      placeholder="enter mock value"
                      onChange={(ev) => onMockDataChange(mock.mock_id, mock.register_id, ev.target.value)}
                    />
                  </TableCell>
                ))}
            </TableRow>

          </TableBody>
        </Table>
      </TableContainer>

      {equationResult && (
        <Typography variant="h6">
          Equation Result = {equationResult}
        </Typography>
      )}

      <Divider />

      <Stack direction={'row'}>
        <Typography
          variant="subtitle1"
        >
          Available Bits:
          {' '}
          {mockScopeDataKeys.map(bitKey => (
            <>
              {bitKey}
              {', '}
            </>
          ))}
        </Typography>
      </Stack>

      <TextField
        id="formula-input"
        label="Formula"
        size="medium"
        variant="standard"
        placeholder="editable formula"
        fullWidth
        multiline
        rows={4}
        onBlur={(ev) => onUpdateFormula(interpreter.interpreter_id, ev.target.value)}
      />

      <Stack
        direction={'row'}
        sx={{
          marginTop: "30px",
          justifyContent: 'space-between',
        }}
      >
        <Button
          variant="contained"
          onClick={() => handleOnEvaluateFormula()}
        >
          Evaluate
        </Button>

        {errorMessage && (
          <Typography
            variant="body1"
            color="error"
          >
            <b>
              Error: {errorMessage}
            </b>
          </Typography>
        )}

      </Stack>
    </>
  )
}
