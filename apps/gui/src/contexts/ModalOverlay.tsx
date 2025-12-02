import { AnimatePresence, motion } from "motion/react";
import React from "react";
import { createContext, useContext, useEffect, useState } from "react";

interface Modal { id: number; content: React.ReactNode }

interface ModalContextType {
    showModal: (content: React.ReactNode) => void;
    hideModal: (id?: number) => void;
}

const ModalContext = createContext<ModalContextType | null>(null);

export const useModal = () => {
    const ctx = useContext(ModalContext)
    if (!ctx) throw new Error("useModal must be used within a ModalProvider")

    return ctx
}

export default function ModalProvider({ children }: { children: React.ReactNode }) {
    const [modals, setModals] = useState<Modal[]>([])
    const [nextId, setNextId] = useState<number>(1)

    const showModal = (content: React.ReactNode) => {
        const id = nextId
        setNextId(n => n + 1)
        setModals(prev => [...prev, { id, content }])
        
        return id
    }

    const hideModal = (id?: number) => {
        if (!id) setModals(prev => prev.slice(0, -1));
        else setModals(prev => prev.filter((m) => m.id !== id))
    }

    useEffect(() => {
        const handleKeyDown = (e: KeyboardEvent) => {
            if (e.key === "Escape" && modals.length > 0) hideModal()
        }

        window.addEventListener("keydown", handleKeyDown);

        return () => window.removeEventListener("keydown", handleKeyDown)
    }, [modals])

    return (
        <ModalContext.Provider value={{ showModal, hideModal }}>
            {children}
            <AnimatePresence>
                {modals.map((modal, idx) => (
                    <motion.div
                        key={modal.id}
                        className="popup-overlay"
                        style={{
                            zIndex: 1000 + idx,
                            pointerEvents: idx === modals.length - 1 ? 'auto' : 'none'
                        }}
                        onClick={() => idx === modals.length - 1 && hideModal(modal.id)}
                        initial={{ opacity: 0 }}
                        animate={{ opacity: 1 }}
                        exit={{ opacity: 0 }}
                        transition={{ duration: 0.2 }}
                    >
                        <motion.div
                            className="popup-content"
                            style={{ pointerEvents: 'auto' }}
                            onClick={(e) => e.stopPropagation()}
                            initial={{ scale: 0.8, opacity: 0 }}
                            animate={{ scale: 1, opacity: 1 }}
                            exit={{ scale: 0.8, opacity: 0 }}
                            transition={{ type: "spring", stiffness: 240, damping: 20 }}
                        >
                            {modal.content}
                        </motion.div>
                    </motion.div>
                ))}
            </AnimatePresence>
        </ModalContext.Provider>
    )
}