use anchor_lang::prelude::*;

// ID único del programa en la red Devnet de Solana
declare_id!("BMaCPqfQwxmhokknch2qykb189FhhfWupxf7uBuoMgx");

#[program]
mod verde_chain {
    use super::*;

    /// -------------------------------------------------------------------------
    /// 1. CREATE (OPERACIÓN 'C'): Inicializar Proyecto
    /// Propósito: Registrar un nuevo proyecto de reforestación y aplicar filtros PLD.
    /// -------------------------------------------------------------------------
    pub fn inicializar_proyecto(
        ctx: Context<CrearProyecto>, 
        nombre_proyecto: String,
        monto_mxn: u64,
        es_efectivo: bool,
        hectareas_meta: u8,
    ) -> Result<()> {
        let proyecto = &mut ctx.accounts.proyecto;

        // --- FILTRO DE AUDITORÍA (Ley Antilavado / PLD) ---
        // Se valida que el monto en efectivo no supere el límite legal de la LFPIORPI ($871,274 MXN).
        if es_efectivo && monto_mxn > 871274 {
            return err!(ErrorCode::ExcesoLimiteEfectivo);
        }

        // --- ASIGNACIÓN DE DATOS (Expediente Digital) ---
        proyecto.hotel = ctx.accounts.usuario.key(); 
        proyecto.nombre = nombre_proyecto;
        proyecto.monto_total = monto_mxn;
        proyecto.es_efectivo = es_efectivo;
        proyecto.hectareas_meta = hectareas_meta;
        
        // El NDVI (Índice de Vegetación) inicia en 0 (sin reporte de éxito aún).
        proyecto.ndvi_actual = 0; 
        // El Escrow inicia bloqueado (false) para proteger el capital.
        proyecto.fondos_liberados = false;

        msg!("C: Proyecto {} creado bajo cumplimiento normativo.", proyecto.nombre);
        Ok(())
    }

    /// -------------------------------------------------------------------------
    /// 2. READ (OPERACIÓN 'R'): Verificar Auditoría
    /// Propósito: Consultar el estatus biológico y financiero actual del registro.
    /// -------------------------------------------------------------------------
    pub fn verificar_auditoria(ctx: Context<ConsultarProyecto>) -> Result<()> {
        let proyecto = &ctx.accounts.proyecto;
        // Se generan logs oficiales de auditoría para lectura externa.
        msg!("R: Consultando Proyecto: {}", proyecto.nombre);
        msg!("Estatus Biológico (NDVI): {}%", proyecto.ndvi_actual);
        msg!("Estado de Fondos: {}", if proyecto.fondos_liberados { "LIBERADOS" } else { "BLOQUEADOS" });
        Ok(())
    }

    /// -------------------------------------------------------------------------
    /// 3. UPDATE (OPERACIÓN 'U'): Actualizar NDVI
    /// Propósito: Registrar el progreso biológico y ejecutar el Bio-Escrow automático.
    /// -------------------------------------------------------------------------
    pub fn actualizar_ndvi(ctx: Context<ActualizarProyecto>, nuevo_ndvi: u8) -> Result<()> {
        let proyecto = &mut ctx.accounts.proyecto;
        proyecto.ndvi_actual = nuevo_ndvi;

        // --- LÓGICA DE BIO-ESCROW (Disparador Automático) ---
        // Si el NDVI reportado por auditoría/satélite es >= 50%, se liberan los fondos.
        if nuevo_ndvi >= 50 {
            proyecto.fondos_liberados = true;
            msg!("U: Éxito biológico confirmado. Fondos desbloqueados automáticamente.");
        } else {
            msg!("U: NDVI actualizado. El mangle aún requiere cuidados para liberar fondos.");
        }
        Ok(())
    }

    /// -------------------------------------------------------------------------
    /// 4. DELETE (OPERACIÓN 'D'): Cerrar Proyecto
    /// Propósito: Eliminar el registro y recuperar el depósito de SOL (Renta).
    /// -------------------------------------------------------------------------
    pub fn cerrar_proyecto(_ctx: Context<CerrarProyecto>) -> Result<()> {
        msg!("D: Registro finalizado. Ciclo de auditoría completado.");
        Ok(())
    }
}

// --- MODELO DE DATOS (ESTRUCTURA DE CUENTAS) ---

#[account]
#[derive(InitSpace)]
pub struct ProyectoMangle {
    pub hotel: Pubkey,            // Identidad del responsable
    #[max_len(30)]
    pub nombre: String,           // Nombre único del proyecto
    pub monto_total: u64,         // Valor financiero en MXN
    pub es_efectivo: bool,        // Bandera de auditoría PLD
    pub hectareas_meta: u8,       // Impacto ecológico planeado
    pub ndvi_actual: u8,          // Indicador de salud vegetal real
    pub fondos_liberados: bool,   // Candado financiero inteligente
}

// --- CONTEXTOS DE OPERACIÓN ---

#[derive(Accounts)]
#[instruction(nombre_proyecto: String)]
pub struct CrearProyecto<'info> {
    #[account(
        init, 
        payer = usuario, 
        space = 8 + ProyectoMangle::INIT_SPACE,
        seeds = [b"proyecto", usuario.key().as_ref(), nombre_proyecto.as_bytes()],
        bump
    )]
    pub proyecto: Account<'info, ProyectoMangle>,
    #[account(mut)]
    pub usuario: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct ConsultarProyecto<'info> {
    pub proyecto: Account<'info, ProyectoMangle>, // Solo lectura
}

#[derive(Accounts)]
pub struct ActualizarProyecto<'info> {
    #[account(mut)]
    pub proyecto: Account<'info, ProyectoMangle>,
    pub usuario: Signer<'info>,
}

#[derive(Accounts)]
pub struct CerrarProyecto<'info> {
    #[account(mut, close = usuario)] // Elimina la cuenta y devuelve el SOL al usuario
    pub proyecto: Account<'info, ProyectoMangle>,
    #[account(mut)]
    pub usuario: Signer<'info>,
}

// --- ERRORES PERSONALIZADOS ---

#[error_code]
pub enum ErrorCode {
    #[msg("ALERTA PLD: El monto en efectivo supera los límites permitidos por la ley mexicana.")]
    ExcesoLimiteEfectivo,
}
}
