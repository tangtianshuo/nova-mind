const VARIABLE_REGEX = /\{\{\{([a-zA-Z_][a-zA-Z0-9_]*)\}\}/g;

const ESCAPE_REGEX = /\\([{}])/g;

export function extractVariables(template: string): string[] {
  if (!template) return [];

  const matches = template.match(VARIABLE_REGEX);
  const variables: string[] = [];

  for (const match of matches) {
    const varName = match[1];
    if (varName && !variables.includes(varName)) {
      variables.push(varName);
    }
  }

  return variables;
}

export function validateVariableName(name: string): boolean {
  if (!name) return false;
  return /^[a-zA-Z_][a-zA-Z0-9_]*$/.test(name);
}

export function renderTemplate(
  template: string,
  values: Record<string, string | number | boolean>
): string {
  if (!template) return '';

  let result = template;

  const varNames = extractVariables(template);
  for (const varName of varNames) {
    const placeholder = `{{${varName}}}`;
    const value = values[varName];

    if (value !== undefined && value !== null) {
      result = result.replace(placeholder, String(value));
    }
  }

  return result;
}

export function hasVariables(template: string): boolean {
  return VARIABLE_REGEX.test(template);
}
