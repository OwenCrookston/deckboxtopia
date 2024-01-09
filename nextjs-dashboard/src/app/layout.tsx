import { Inter } from 'next/font/google'
import { Grid } from '@mui/material'

const inter = Inter({ subsets: ['latin'] })

export default function RootLayout({
  children,
}: {
  children: React.ReactNode
}) {
  return (
    <Grid sx={{ width: '100vw', height: '100vh' }} display='flex' flexDirection={'column'} justifyContent={'center'} alignItems={'center'}>
      {children}
    </Grid>
  )
}
