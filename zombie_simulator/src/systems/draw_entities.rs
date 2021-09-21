use eyre::Result;
use ggez::graphics::DrawParam;
use ggez::Context;

use jecs::World;

use crate::components::location::Location;
use crate::resources::entity_mesh::EntityMesh;

pub struct DrawEntities;

impl DrawEntities {
    pub fn run(world: &World, ctx: &mut Context) -> Result<()> {
        let mesh = world.get_resource::<EntityMesh>().unwrap();

        let query = world.query().with_component::<Location>()?.run();

        for location in query.result.first().unwrap() {
            let borrowed_location = location.borrow();
            let location = borrowed_location.downcast_ref::<Location>().unwrap();

            ggez::graphics::draw(
                ctx,
                &mesh.0,
                DrawParam::new().dest(location.to_mint_vector2()),
            )?;
        }

        Ok(())
    }
}
