import { reactive, computed } from 'vue';
import type { SkillVariable, VariableFormValues } from '@/types';
import { useVariableValidation } from './useVariableValidation';

const VARIABLE_REGEX = /\{\{([a-zA-Z_][a-zA-Z0-9]*)\}\}/g;

export function extractVariables(template: string): string[] {
  const matches = template.matchAll(VARIABLE_REGEX);
  const variables = new Set<string>();
  for (const match of matches) {
    variables.add(match[1]);
  }
  return Array.from(variables);
}

export function renderTemplate(
  template: string,
  values: VariableFormValues
): string {
  return template.replace(VARIABLE_REGEX, (_, varName) => {
    const value = values[varName];
    if (value === undefined || value === null) {
      return '';
    }
    return String(value);
  });
}

export function validateVariableName(name: string): boolean {
  return /^[a-zA-Z_][a-zA-Z0-9_]*$/.test(name);
}

export function useVariableForm(variables: SkillVariable[]) {
  const formValues = reactive<VariableFormValues>({});
  const { errors, validate, validateSingle, clearErrors, getError, hasErrors } = 
        useVariableValidation(variables);

    function initializeDefaults() {
        for (const variable of variables) {
            if (variable.default !== undefined) {
                formValues[variable.name] = variable.default;
            } else {
                switch (variable.type) {
                    case 'checkbox':
                        formValues[variable.name] = false;
                        break;
                    case 'number':
                        formValues[variable.name] = variable.validation?.min ?? 0;
                        break;
                    default:
                        formValues[variable.name] = '';
                }
            }
        }
    }

    initializeDefaults();

    const isRequiredFilled = computed(() => {
        return variables.every(v => {
            if (!v.required) return true;
            const value = formValues[v.name];
            return value !== undefined && value !== null && value !== '';
        });
    });

    const isValid = computed(() => {
        return isRequiredFilled.value && !hasErrors.value;
    });

    function setValue(name: string, value: string | number | boolean) {
        formValues[name] = value;
        validateSingle(name, value);
    }

    function getValue(name: string): string | number | boolean | undefined {
        return formValues[name];
    }

    function reset() {
        clearErrors();
        initializeDefaults();
    }

    function getValues(): VariableFormValues {
        return { ...formValues };
    }

    function setValues(values: VariableFormValues) {
        Object.assign(formValues, values);
    }

    return {
        formValues,
        errors,
        isValid,
        isRequiredFilled,
        hasErrors,
        setValue,
        getValue,
        reset,
        getValues,
        setValues,
        validate,
        getError,
        clearErrors,
    };
}
