use anchor_lang::prelude::*;

// Esta ID se generará cuando hagas el primer 'anchor build'
declare_id!("BMaCPqfQwxmhokknch2qykb189FhhfWupxf7uBuoMgx");

#[program]
mod verde_chain {
    use super::*;

    // PASO 1: Crear el Expediente del Proyecto (Create)
    pub fn inicializar_proyecto(
        ctx: Context<CrearProyecto>, 
        nombre_proyecto: String,
        monto_mxn: u64,
        es_efectivo: bool,
        hectareas_meta: u8,
    ) -> Result<()> {
        let proyecto = &mut ctx.accounts.proyecto;

        // --- VALIDACIÓN DE AUDITORÍA (Triple Filtro) ---
        
        // 1. Filtro PLD: Límite legal de efectivo ($871,274 MXN)
        if es_efectivo && monto_mxn > 871274 {
            return err!(ErrorCode::ExcesoLimiteEfectivo);
        }

        // 2. Filtro Greenwashing: Ratio biológico (Monto excesivo para pocas hectáreas)
        // Si el costo por hectárea es > 500,000 y solo hay 1, sospechamos.
        if monto_mxn > 500000 && hectareas_meta < 2 {
            return err!(ErrorCode::SospechaGreenwashing);
        }

        require!(nombre_proyecto.len() <= 30, ErrorCode::NombreMuyLargo);

        // Inicializar datos en la cuenta (PDA)
        proyecto.hotel = ctx.accounts.usuario.key();
        proyecto.nombre = nombre_proyecto;
        proyecto.monto_total = monto_mxn;
        proyecto.es_efectivo = es_efectivo;
        proyecto.hectareas_meta = hectareas_meta;
        proyecto.ndvi_actual = 0; // Inicia en 0 (sin vegetación confirmada)
        proyecto.fondos_liberados = false;

        msg!("Proyecto {} creado exitosamente bajo auditoría.", proyecto.nombre);
        Ok(())
    }

    // PASO 2: Actualizar Hitos Biológicos (Update)
    // Solo el oráculo (autoridad) debería poder llamar a esto
    pub fn actualizar_ndvi(ctx: Context<ActualizarProyecto>, nuevo_ndvi: u8) -> Result<()> {
        let proyecto = &mut ctx.accounts.proyecto;
        
        proyecto.ndvi_actual = nuevo_ndvi;

        // Si el NDVI llega a 50 (50% de salud biómata), se marca como apto para liberar fondos
        if nuevo_ndvi >= 50 {
            proyecto.fondos_liberados = true;
            msg!("Hito biológico cumplido. Fondos listos para el proveedor.");
        }

        Ok(())
    }
}

#[error_code]
pub enum ErrorCode {
    #[msg("El nombre del proyecto es demasiado largo (Max 30).")]
    NombreMuyLargo,
    #[msg("ALERTA PLD: Los pagos en efectivo no pueden superar los $871,274 MXN.")]
    ExcesoLimiteEfectivo,
    #[msg("ALERTA BIOLÓGICA: El presupuesto no coincide con el impacto ambiental (Greenwashing).")]
    SospechaGreenwashing,
}

#[account]
#[derive(InitSpace)]
pub struct ProyectoMangle {
    pub hotel: Pubkey,            // Dueño del proyecto (32 bytes)
    #[max_len(30)]
    pub nombre: String,           // Nombre (30 bytes)
    pub monto_total: u64,         // Monto en MXN
    pub es_efectivo: bool,        // Bandera de auditoría
    pub hectareas_meta: u8,       // Impacto biológico
    pub ndvi_actual: u8,          // Salud del mangle (0-100)
    pub fondos_liberados: bool,   // Estado del Escrow
}

#[derive(Accounts)]
#[instruction(nombre_proyecto: String)]
pub struct CrearProyecto<'info> {
    #[account(
        init, 
        payer = usuario, 
        space = 8 + ProyectoMangle::INIT_SPACE,
        // Usamos el nombre del proyecto como semilla para que cada uno tenga su propia PDA
        seeds = [b"proyecto", usuario.key().as_ref(), nombre_proyecto.as_bytes()],
        bump
    )]
    pub proyecto: Account<'info, ProyectoMangle>,
    
    #[account(mut)]
    pub usuario: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct ActualizarProyecto<'info> {
    #[account(mut)]
    pub proyecto: Account<'info, ProyectoMangle>,
    pub usuario: Signer<'info>,
}