import { AutoField, AutoFieldProps } from "uniforms-mui";
import { SchemaKeys } from "../schema";

export const TypedAutoField = (props: AutoFieldProps & { name: SchemaKeys }) => (
  <AutoField {...props} />
)
