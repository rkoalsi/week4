import { useFormik } from 'formik';
import * as yup from 'yup';
import TextField from '@mui/material/TextField';
import Button from '@mui/material/Button';
import Stack from '@mui/material/Stack';
interface MyFormProps {
  greet: (str: string) => void;
  children?: React.ReactNode;
}
const validationSchema = yup.object({
  firstName: yup.string().required('First name is required'),
  age: yup.number().required('Age is required'),
  address: yup.string().required('Address is required'),
  greetText: yup.string().required('Greet Text is required'),
});

const MyForm: React.FC<MyFormProps> = (props) => {
  const formik = useFormik({
    initialValues: {
      firstName: 'Pablo',
      age: 24,
      address: '0xDeadBeeef0000000',
      greetText: 'Hello World',
    },
    validationSchema: validationSchema,
    onSubmit: (values) => {
      console.log(JSON.stringify(values, null, 2));
      props.greet(values.greetText);
    },
  });

  return (
    <div style={{ marginTop: '1rem' }}>
      <form onSubmit={formik.handleSubmit}>
        <Stack spacing={2}>
          <TextField
            color={'secondary'}
            id='firstName'
            name='firstName'
            required
            label='First Name'
            value={formik.values.firstName}
            onChange={formik.handleChange}
            error={formik.touched.firstName && Boolean(formik.errors.firstName)}
            helperText={formik.touched.firstName && formik.errors.firstName}
          />
          <TextField
            id='age'
            required
            name='age'
            type={'number'}
            color={'secondary'}
            label='Age'
            value={formik.values.age}
            onChange={formik.handleChange}
            error={formik.touched.age && Boolean(formik.errors.age)}
            helperText={formik.touched.age && formik.errors.age}
          />
          <TextField
            required
            id='address'
            name='address'
            color={'secondary'}
            label='Address'
            value={formik.values.address}
            onChange={formik.handleChange}
            error={formik.touched.address && Boolean(formik.errors.address)}
            helperText={formik.touched.address && formik.errors.address}
          />
          <TextField
            id='greetText'
            name='greetText'
            color={'secondary'}
            label='Greet Text'
            value={formik.values.greetText}
            onChange={formik.handleChange}
            error={formik.touched.greetText && Boolean(formik.errors.greetText)}
            helperText={formik.touched.greetText && formik.errors.greetText}
          />
          <Button color='primary' variant='contained' fullWidth type='submit'>
            Submit
          </Button>
        </Stack>
      </form>
    </div>
  );
};

export default MyForm;
