import { ref, computed } from 'vue';
import type { SkillVariable, VariableFormValues } from '@/types';

export interface ValidationError {
  field: string;
  message: string;
}

export function useVariableValidation(variables: SkillVariable[]) {
  const errors = ref<Record<string, string>>({});

  function validateField(
    variable: SkillVariable,
    value: string | number | boolean | undefined
  ): string | null {
    if (variable.required) {
      if (value === undefined || value === null || value === '') {
        return `${variable.label}是必填项`;
      }
    }

    if (value === undefined || value === null || value === '') {
      return null;
    }

    const strValue = String(value);

    if (variable.validation) {
      const v = variable.validation;

      if (v.minLength !== undefined && strValue.length < v.minLength) {
        return v.message || `${variable.label}长度不能少于${v.minLength}个字符`;
      }

      if (v.maxLength !== undefined && strValue.length > v.maxLength) {
        return v.message || `${variable.label}长度不能超过${v.maxLength}个字符`;
      }

      if (variable.type === 'number') {
        const numValue = Number(value);
        if (isNaN(numValue)) {
          return `${variable.label}必须是有效的数字`;
        }
        if (v.min !== undefined && numValue < v.min) {
          return v.message || `${variable.label}不能小于${v.min}`;
        }
        if (v.max !== undefined && numValue > v.max) {
          return v.message || `${variable.label}不能大于${v.max}`;
        }
      }

      if (v.pattern) {
        const regex = new RegExp(v.pattern);
        if (!regex.test(strValue)) {
          return v.message || `${variable.label}格式不正确`;
        }
      }
    }

    if (variable.type === 'select' && variable.options) {
      const validValues = variable.options.map(opt => opt.value);
      if (!validValues.includes(strValue)) {
        return `${variable.label}的值无效`;
      }
    }

    return null;
  }

  function validate(values: VariableFormValues): boolean {
    errors.value = {};
    let isValid = true;

    for (const variable of variables) {
      const error = validateField(variable, values[variable.name]);
      if (error) {
        errors.value[variable.name] = error;
        isValid = false;
      }
    }

    return isValid;
  }

  function validateSingle(
    name: string,
    value: string | number | boolean | undefined
  ): boolean {
    const variable = variables.find(v => v.name === name);
    if (!variable) return true;

    const error = validateField(variable, value);
    if (error) {
      errors.value[name] = error;
      return false;
    } else {
      delete errors.value[name];
      return true;
    }
  }

  function clearErrors() {
    errors.value = {};
  }

  function getError(name: string): string | undefined {
    return errors.value[name];
  }

  const hasErrors = computed(() => Object.keys(errors.value).length > 0);

  return {
    errors,
    hasErrors,
    validate,
    validateSingle,
    validateField,
    clearErrors,
    getError,
  };
}
