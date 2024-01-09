import { DeckboxtopiaCard } from "@/app/types/card";
import {
  Card,
  CardContent,
  CardMedia,
  Typography,
} from "@mui/material";

export const CardTile = ({ card }: { card: DeckboxtopiaCard }) => {
  return (
    <Card sx={{ maxWidth: 345 }}>
      <CardMedia
        sx={{ height: 140 }}
        image={card.art_url}
        title={card.name}
      />
      <CardContent>
        <Typography gutterBottom variant="h5" component="div">
          {card.name}
        </Typography>
      </CardContent>
    </Card>
  )
};