use anchor_lang::prelude::*;

declare_id!("BMaCPqfQwxmhokknch2qykb189FhhfWupxf7uBuoMgx");

#[program]
mod verde_chain {
    use super::*;

    // --- FUNCIÓN: INICIALIZAR PROYECTO ---
    // Aquí es donde se aplican los filtros de cumplimiento (Compliance) antes de crear el registro.
    pub fn inicializar_proyecto(
        ctx: Context<CrearProyecto>, 
        nombre_proyecto: String,
        monto_mxn: u64,
        es_efectivo: bool,
        hectareas_meta: u8,
    ) -> Result<()> {
        let proyecto = &mut ctx.accounts.proyecto;

        // --- 1. FILTRO ANTILAVADO (PLD / PITUFEO) ---
        // La ley mexicana prohíbe pagar ciertos servicios en efectivo si superan los $871,274 MXN.
        // Aquí el código bloquea la transacción si se intenta registrar un pago ilegal.
        if es_efectivo && monto_mxn > 871274 {
            return err!(ErrorCode::ExcesoLimiteEfectivo);
        }

        // --- 2. FILTRO ANTI-GREENWASHING ---
        // ¿Para qué sirve?: Evita que se simulen proyectos ambientales caros que no tienen impacto real.
        // Si el monto es muy alto (> $500k) pero la meta biológica es casi nula (< 2 hectáreas),
        // el código sospecha de un fraude ecológico y aborta el proceso.
        if monto_mxn > 500000 && hectareas_meta < 2 {
            return err!(ErrorCode::SospechaGreenwashing);
        }

        require!(nombre_proyecto.len() <= 30, ErrorCode::NombreMuyLargo);

        // --- 3. CREACIÓN DEL EXPEDIENTE DIGITAL ---
        proyecto.hotel = ctx.accounts.usuario.key();
        proyecto.nombre = nombre_proyecto;
        proyecto.monto_total = monto_mxn;
        proyecto.es_efectivo = es_efectivo;
        proyecto.hectareas_meta = hectareas_meta;
        
        // El NDVI inicia en 0 porque apenas se va a plantar o el contrato acaba de iniciar.
        proyecto.ndvi_actual = 0; 
        
        // Los fondos están congelados (false) hasta que la naturaleza cumpla su parte.
        proyecto.fondos_liberados = false;

        msg!("Proyecto {} creado exitosamente bajo auditoría.", proyecto.nombre);
        Ok(())
    }

    // --- FUNCIÓN: ACTUALIZAR NDVI (HITOS BIOLÓGICOS) ---
    // ¿Qué es el NDVI?: Es un índice que mide la salud de la vegetación (de 0 a 100 en este código).
    // ¿Para qué sirve?: Funciona como un oráculo de verdad. Si el NDVI es alto, el mangle está vivo.
    pub fn actualizar_ndvi(ctx: Context<ActualizarProyecto>, nuevo_ndvi: u8) -> Result<()> {
        let proyecto = &mut ctx.accounts.proyecto;
        
        // Aquí se "escribe" el progreso biológico reportado por el satélite o auditor.
        proyecto.ndvi_actual = nuevo_ndvi;

        // --- CÁLCULO DEL HITO BIOLÓGICO ---
        // Este es el "Bio-Escrow": El dinero se libera automáticamente SOLO si el NDVI llega a 50.
        // Si el proveedor no cuida el mangle y el NDVI nunca sube, nunca cobra el resto del dinero.
        if nuevo_ndvi >= 50 {
            proyecto.fondos_liberados = true;
            msg!("Hito biológico cumplido. Fondos listos para el proveedor.");
        }

        Ok(())
    }
}

// --- DEFINICIÓN DE LA CUENTA (ALMACENAMIENTO) ---
#[account]
#[derive(InitSpace)]
pub struct ProyectoMangle {
    pub hotel: Pubkey,            
    #[max_len(30)]
    pub nombre: String,           
    pub monto_total: u64,         
    pub es_efectivo: bool,        
    pub hectareas_meta: u8,       
    pub ndvi_actual: u8,          // Aquí se guarda la "salud" del proyecto
    pub fondos_liberados: bool,   // Este es el candado financiero
}